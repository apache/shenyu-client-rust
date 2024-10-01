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

use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct MetaInfo {
    pub path: String,
}

/// {
/// "appName":"springCloud-test",
/// "contextPath":"/springcloud",
/// "path":"/springcloud/order/path/{id}/name",
/// "pathDesc":"",
/// "rpcType":"springCloud",
/// "serviceName":"org.apache.shenyu.examples.springcloud.controller.OrderController",
/// "methodName":"testRestFul",
/// "rule_name":"/springcloud/order/path/{id}/name",
/// "parameterTypes":"java.lang.String",
/// "enabled":true,
/// "pluginNames":[],
/// "registerMetaData":false,
/// "timeMillis":1724062308618,
/// "addPrefixed":false
/// }

#[derive(Debug, Clone)]
pub struct UriInfo {
    pub path: String,
    pub rule_name: String,
    pub service_name: Option<String>,
    pub method_name: String,
}

pub enum EventType {
    REGISTER,

    UPDATED,

    DELETED,

    IGNORED,

    OFFLINE,
}

impl Display for EventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::REGISTER => write!(f, "REGISTER"),
            EventType::UPDATED => write!(f, "UPDATED"),
            EventType::DELETED => write!(f, "DELETED"),
            EventType::IGNORED => write!(f, "IGNORED"),
            EventType::OFFLINE => write!(f, "OFFLINE"),
        }
    }
}
