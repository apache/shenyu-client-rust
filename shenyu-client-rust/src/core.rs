// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use crate::config::ShenYuConfig;
use crate::error::ShenYuError;
use crate::model::{EventType, UriInfo};
use dashmap::DashMap;
use serde_json::Value;
use std::io::{Error, ErrorKind};
use std::net::IpAddr;
use tracing::{error, info, warn};
use ureq::OrAnyStatus;

pub const REGISTER_META_DATA_SUFFIX: &str = "/shenyu-client/register-metadata";
pub const REGISTER_URI_SUFFIX: &str = "/shenyu-client/register-uri";
pub const REGISTER_DISCOVERY_CONFIG_SUFFIX: &str = "/shenyu-client/register-discoveryConfig";
pub const REGISTER_OFFLINE_SUFFIX: &str = "/shenyu-client/offline";
pub const PLATFORM_LOGIN_SUFFIX: &str = "/platform/login";

#[derive(Debug)]
#[warn(dead_code)]
pub struct ShenyuClient {
    pub(super) headers: DashMap<String, String>,
    app_name: String,
    env: ShenYuConfig,
    host: Option<String>,
    port: u16,
    gateway_base_urls: Vec<String>,
    register_meta_data_path_list: Vec<String>,
    register_uri_list: Vec<String>,
    register_token_servers: Vec<String>,
    register_discover_config_servers: Vec<String>,
    register_offline_servers: Vec<String>,
    uri_infos: Vec<UriInfo>,
}

impl ShenyuClient {
    pub fn register(&self) -> Result<(), Error> {
        if let Ok(token) = self.get_register_token() {
            self.headers
                .insert("X-Access-Token".to_string(), token.to_string());
        } else {
            return Err(Error::new(ErrorKind::Other, "Can't get register token"));
        }
        self.register_all_metadata(true)
            .expect("Failed to register metadata");
        self.register_uri().expect("Failed to register URI");
        self.register_discovery_config()
            .expect("Failed to register discovery config");
        Ok(())
    }

    pub fn new(
        config: ShenYuConfig,
        app_name: &str,
        uri_infos: &[UriInfo],
        port: u16,
    ) -> Result<Self, String> {
        let headers = DashMap::new();
        headers.insert(
            "Content-Type".to_string(),
            "application/json;charset=UTF-8".to_string(),
        );
        let mut client = ShenyuClient {
            headers,
            app_name: app_name.to_string(),
            env: config,
            host: None,
            port,
            gateway_base_urls: vec![],
            register_meta_data_path_list: vec![],
            register_uri_list: vec![],
            register_token_servers: vec![],
            register_discover_config_servers: vec![],
            register_offline_servers: vec![],
            uri_infos: uri_infos.to_owned(),
        };
        client.set_up_gateway_service_url()?;
        Ok(client)
    }
}

impl ShenyuClient {
    fn set_up_gateway_service_url(&mut self) -> Result<(), String> {
        self.gateway_base_urls = self
            .env
            .register
            .servers
            .split(',')
            .map(|s| s.to_string())
            .collect();

        self.register_meta_data_path_list = self
            .gateway_base_urls
            .iter()
            .map(|url| format!("{}{}", url, REGISTER_META_DATA_SUFFIX))
            .collect();
        self.register_uri_list = self
            .gateway_base_urls
            .iter()
            .map(|url| format!("{}{}", url, REGISTER_URI_SUFFIX))
            .collect();
        self.register_token_servers = self
            .gateway_base_urls
            .iter()
            .map(|url| format!("{}{}", url, PLATFORM_LOGIN_SUFFIX))
            .collect();
        self.register_discover_config_servers = self
            .gateway_base_urls
            .iter()
            .map(|url| format!("{}{}", url, REGISTER_DISCOVERY_CONFIG_SUFFIX))
            .collect();
        self.register_offline_servers = self
            .gateway_base_urls
            .iter()
            .map(|url| format!("{}{}", url, REGISTER_OFFLINE_SUFFIX))
            .collect();

        let mut host = None;
        #[cfg(not(target_os = "macos"))]
        {
            host = match local_ip_address::local_ip() {
                Ok(IpAddr::V4(ipv4)) => Some(IpAddr::V4(ipv4)),
                Ok(IpAddr::V6(ipv6)) => Some(IpAddr::from(ipv6.to_ipv4().unwrap())),
                _ => None,
            };
        }
        #[cfg(target_os = "macos")]
        {
            use local_ip_address::macos;
            for (_, ipaddr) in macos::list_afinet_netifas().unwrap() {
                if IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)).eq(&ipaddr) {
                    continue;
                }
                host = match ipaddr {
                    IpAddr::V4(ipv4) => Some(IpAddr::from(ipv4)),
                    _ => continue,
                };
            }
        }
        if let Some(host) = host {
            self.host = Some(host.to_string());
            Ok(())
        } else {
            Err("Failed to determine local IP address".to_string())
        }
    }

    fn request(&self, url: &str, json_data: &Value) -> Result<bool, Error> {
        let mut builder = ureq::post(url);
        // 遍历header， 添加到builder中
        for r in &self.headers {
            builder = builder.set(r.key(), r.value());
        }
        let res = builder.send_json(json_data).unwrap();
        let status_code = res.status();
        let msg = res.into_string()?;

        if msg == "success" {
            Ok(true)
        } else {
            warn!(
                "Request ({}) failed, status code: {}, msg: {}",
                url, status_code, msg
            );
            Ok(false)
        }
    }

    pub(crate) fn get_register_token(&self) -> Result<String, Error> {
        let hashmap = &self.env.register.props;
        let params = [
            ("userName", hashmap.get("username").unwrap().as_str()),
            ("password", hashmap.get("password").unwrap().as_str()),
        ];

        let result = Err(ShenYuError::new(500, "Can't get register token".to_string()).into());
        for url in &self.register_token_servers {
            let res_data: Value = ureq::get(url)
                .query_pairs(params)
                .call()
                .or_any_status()
                .map_err(|e| Error::new(ErrorKind::Other, format!("{e}")))?
                .into_json()?;
            match res_data
                .get("data")
                .and_then(|data| data.get("token"))
                .and_then(|token| token.as_str())
            {
                Some(token) => return Ok(token.to_string()),
                None => continue,
            }
        }
        result
    }

    pub fn register_uri(&self) -> Result<bool, Error> {
        let app_name = &self.app_name.clone();
        let rpc_type = &self.env.uri.rpc_type.clone();
        let context_path = &self.env.uri.context_path.clone();

        let port = &self.port;
        let host = &self.host;

        let json_data = serde_json::json!({
            "appName": app_name,
            "contextPath": context_path,
            "protocol": rpc_type,
            "rpcType": rpc_type,
            "host": host.clone().unwrap(),
            "port": port,
            "eventType": EventType::REGISTER.to_string(),
        });

        for url in &self.register_uri_list {
            if self.request(url, &json_data)? {
                info!(
                    "[SUCCESS], register uri success, register data: {:#?}",
                    json_data
                );
                return Ok(true);
            }
        }

        error!(
            "[ERROR], register uri failed, app_name: {}, host: {}, port: {}",
            app_name,
            host.clone().unwrap(),
            port
        );
        Ok(false)
    }

    pub fn register_all_metadata(&self, enabled: bool) -> Result<bool, Error> {
        for x in self.uri_infos.iter() {
            match self.register_metadata(
                false,
                Some(&x.path),
                Some(&x.method_name),
                Some(&x.rule_name),
                enabled,
            ) {
                Ok(true) => continue,
                Ok(false) => return Ok(false),
                Err(e) => return Err(e),
            }
        }
        Ok(true)
    }

    fn register_metadata(
        &self,
        register_all: bool,
        path: Option<&str>,
        method: Option<&str>,
        rule_name: Option<&str>,
        enabled: bool,
    ) -> Result<bool, Error> {
        let context_path = &self.env.uri.context_path.clone();
        let app_name = &self.app_name.clone();
        let rpc_type = &self.env.uri.rpc_type.clone();
        let path = if register_all {
            format!("{}**", context_path)
        } else {
            path.unwrap_or("").to_string()
        };

        let rule_name = rule_name.unwrap_or(&path).to_string();
        let json_data = serde_json::json!({
            "appName": app_name.clone(),
            "contextPath": context_path.clone(),
            "path": context_path.clone() + path.as_str(),
            "pathDesc": "",
            "rpcType": rpc_type,
            "ruleName": context_path.clone() + rule_name.as_str(),
            "serviceName": app_name.clone(),
            "methodName": method.unwrap_or("").to_string(),
            "parameterTypes": "",
            "rpcExt": "",
            "host": self.host.clone().unwrap(),
            "port": self.port,
            "enabled": enabled,
            "registerMetaData": "",
            "pluginNames": []
        });

        for url in &self.register_meta_data_path_list {
            if self.request(url, &json_data)? {
                info!(
                    "[SUCCESS], register metadata success, register data: {:#?}",
                    json_data
                );
                return Ok(true);
            }
        }

        error!(
            "[ERROR], register metadata failed, app_name: {}, path: {}, contextPath: {}",
            app_name, path, context_path
        );
        Ok(false)
    }

    pub fn register_discovery_config(&self) -> Result<bool, Error> {
        let discovery_type = &self.env.discovery.discovery_type.clone();
        let register_path = &self.env.discovery.register_path.clone();
        let server_lists = &self.env.discovery.server_lists.clone();
        let props = &self.env.discovery.props.clone();
        let plugin_name = &self.env.discovery.plugin_name.clone();
        let context_path = &self.env.uri.context_path.clone();

        let port = &self.port;
        let host = &self.host;

        let json_data = serde_json::json!({
            "name": "default".to_string() + discovery_type,
            "selectorName": context_path,
            "handler": "{}",
            "listenerNode":register_path,
            "serverList": server_lists,
            "props": props,
            "discoveryType": discovery_type.clone(),
            "pluginName": plugin_name,
        });

        for url in &self.register_discover_config_servers {
            if self.request(url, &json_data)? {
                info!(
                    "[SUCCESS], register discover config success, register data: {:#?}",
                    json_data
                );
                return Ok(true);
            }
        }

        error!(
            "[ERROR], register discover config failed, discovery_type: {}, host: {}, port: {}",
            discovery_type,
            host.clone().unwrap(),
            port
        );
        Ok(false)
    }

    pub fn offline_register(&self) {
        let app_name = &self.app_name.clone();
        let rpc_type = &self.env.uri.rpc_type.clone();
        let context_path = &self.env.uri.context_path.clone();

        let port = &self.port;
        let host = &self.host;

        let json_data = serde_json::json!({
            "appName": app_name,
            "contextPath": context_path,
            "protocol": rpc_type,
            "host": host.clone().unwrap(),
            "port": port,
            "eventType": EventType::REGISTER.to_string(),
        });

        for url in &self.register_offline_servers {
            if self.request(url, &json_data).unwrap() {
                info!(
                    "[SUCCESS], offline success, register data: {:#?}",
                    json_data
                );
                return;
            }
        }

        error!(
            "[ERROR], register uri failed, app_name: {}, host: {}, port: {}",
            app_name,
            host.clone().unwrap(),
            port
        );
    }
}
