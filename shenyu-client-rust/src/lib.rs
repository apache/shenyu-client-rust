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

#![deny(
    // The following are allowed by default lints according to
    // https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html
    absolute_paths_not_starting_with_crate,
    explicit_outlives_requirements,
    macro_use_extern_crate,
    redundant_lifetimes,
    anonymous_parameters,
    bare_trait_objects,
    // elided_lifetimes_in_paths, // allow anonymous lifetime
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    // single_use_lifetimes, // TODO: fix lifetime names only used once
    // trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    // unsafe_code,
    unstable_features,
    // unused_crate_dependencies,
    unused_lifetimes,
    unused_macro_rules,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    variant_size_differences,

    warnings, // treat all wanings as errors

    clippy::all,
    // clippy::restriction,
    clippy::pedantic,
    // clippy::nursery, // It's still under development
    clippy::cargo,
)]
#![allow(
    // Some explicitly allowed Clippy lints, must have clear reason to allow
    clippy::blanket_clippy_restriction_lints, // allow clippy::restriction
    clippy::implicit_return, // actually omitting the return keyword is idiomatic Rust code
    clippy::module_name_repetitions, // repeation of module name in a struct name is not big deal
    clippy::multiple_crate_versions, // multi-version dependency crates is not able to fix
    clippy::missing_errors_doc, // TODO: add error docs
    clippy::missing_panics_doc, // TODO: add panic docs
    clippy::panic_in_result_fn,
    clippy::shadow_same, // Not too much bad
    clippy::shadow_reuse, // Not too much bad
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::indexing_slicing,
    clippy::separated_literal_suffix, // conflicts with clippy::unseparated_literal_suffix
    clippy::single_char_lifetime_names, // TODO: change lifetime names
)]
#![doc = include_str!("../README.md")]

use crate::model::UriInfo;

/// A mod for CI.
pub mod ci;
/// Config structs.
pub mod config;
/// Shenyu client core.
pub mod core;
/// Error handling.
pub mod error;
/// Macros.
pub mod macros;
/// Structs.
pub mod model;

#[allow(missing_docs)]
pub trait IRouter {
    /// Get app name.
    fn app_name(&self) -> &str;

    /// Get uri infos.
    fn uri_infos(&self) -> &Vec<UriInfo>;
}

#[allow(missing_docs)]
#[cfg(feature = "axum")]
pub mod axum_impl {
    use super::model::UriInfo;
    use crate::IRouter;
    use axum::extract::Request;
    use axum::response::IntoResponse;
    use axum::routing::MethodRouter;
    use axum::Router;
    use std::convert::Infallible;
    use tower_service::Service;

    /// A router that can be used to register routes.
    ///
    /// This is a wrapper around `Router` that provides a more ergonomic API.
    /// It allows you to define routes and nest other routers or services.
    ///
    /// # Fields
    ///
    /// * `app_name` - The name of the application.
    /// * `uri_infos` - A vector of URI information.
    ///
    /// # Examples
    /// ```rust
    ///
    /// use axum::routing::{get, post};
    /// use shenyu_client_rust::axum_impl::ShenYuRouter;
    ///
    /// async fn health_handler() -> &'static str {
    ///     "OK"
    /// }
    ///
    /// async fn create_user_handler() -> &'static str {
    ///     "User created"
    /// }
    ///
    /// async fn not_found_handler() -> &'static str {
    ///     "Not found"
    /// }
    ///
    /// let app = ShenYuRouter::<()>::new("shenyu_client_app")
    ///     .nest("/api", ShenYuRouter::new("api"))
    ///     .route("/health", "get", get(health_handler))
    ///     .route("/users", "post", post(create_user_handler));
    ///
    /// ```
    ///
    #[derive(Debug, Clone)]
    pub struct ShenYuRouter<S = ()> {
        app_name: String,
        inner: Router<S>,
        uri_infos: Vec<UriInfo>,
    }

    impl<S> ShenYuRouter<S>
    where
        S: Clone + Send + Sync + 'static,
    {
        #[must_use]
        pub fn new(app_name: &str) -> Self {
            Self {
                app_name: app_name.to_string(),
                inner: Router::new(),
                uri_infos: Vec::new(),
            }
        }

        #[must_use]
        pub fn uri_info(mut self, uri_info: UriInfo) -> Self {
            self.uri_infos.push(uri_info);
            self
        }

        #[must_use]
        pub fn route(mut self, path: &str, method: &str, method_router: MethodRouter<S>) -> Self {
            self.inner = self.inner.route(path, method_router);
            self.uri_infos.push(UriInfo {
                path: path.to_string(),
                rule_name: path.to_string(),
                service_name: None,
                method_name: method.to_string(),
            });
            self
        }

        #[must_use]
        pub fn route_service<T>(mut self, path: &str, method: &str, service: T) -> Self
        where
            T: Service<Request, Error = Infallible> + Clone + Send + 'static,
            T::Response: IntoResponse,
            T::Future: Send + 'static,
        {
            self.inner = self.inner.route_service(path, service);
            self.uri_infos.push(UriInfo {
                path: path.to_string(),
                rule_name: path.to_string(),
                service_name: None,
                method_name: method.to_string(),
            });
            self
        }

        #[must_use]
        #[track_caller]
        pub fn nest(mut self, path: &str, route: ShenYuRouter<S>) -> Self {
            self.inner = self.inner.nest(path, route.inner);
            self.uri_infos.extend(route.uri_infos);
            self
        }

        #[must_use]
        #[track_caller]
        pub fn nest_service<T>(mut self, path: &str, method: &str, service: T) -> Self
        where
            T: Service<Request, Error = Infallible> + Clone + Send + 'static,
            T::Response: IntoResponse,
            T::Future: Send + 'static,
        {
            self.inner = self.inner.nest_service(path, service);
            self.uri_infos.push(UriInfo {
                path: path.to_string(),
                rule_name: path.to_string(),
                service_name: None,
                method_name: method.to_string(),
            });
            self
        }

        #[must_use]
        pub fn uri_infos(&self) -> &Vec<UriInfo> {
            &self.uri_infos
        }

        #[must_use]
        #[track_caller]
        pub fn merge<R>(mut self, other: ShenYuRouter<R>) -> Self
        where
            R: Into<Router<S>>,
            S: Clone + Send + Sync + 'static,
            Router<S>: From<Router<R>>,
        {
            self.inner = self.inner.merge(other.inner);
            self.uri_infos.extend(other.uri_infos);
            self
        }
    }

    impl<S> From<ShenYuRouter<S>> for Router<S>
    where
        S: Clone + Send + Sync + 'static,
    {
        fn from(val: ShenYuRouter<S>) -> Self {
            val.inner
        }
    }

    impl<S> IRouter for ShenYuRouter<S> {
        fn app_name(&self) -> &str {
            &self.app_name
        }

        fn uri_infos(&self) -> &Vec<UriInfo> {
            &self.uri_infos
        }
    }
}

#[allow(missing_docs)]
#[cfg(feature = "actix-web")]
pub mod actix_web_impl {
    use super::model::UriInfo;
    use crate::IRouter;

    /// A router that can be used to register routes.
    ///
    /// This is a wrapper around `Router` that provides a more ergonomic API.
    /// It allows you to define routes and nest other routers or services.
    ///
    /// # Fields
    ///
    /// * `app_name` - The name of the application.
    /// * `uri_infos` - A vector of URI information.
    #[derive(Debug, Clone)]
    pub struct ShenYuRouter {
        app_name: String,
        uri_infos: Vec<UriInfo>,
    }

    impl ShenYuRouter {
        #[must_use]
        pub fn new(app_name: &str) -> Self {
            Self {
                app_name: app_name.to_string(),
                uri_infos: Vec::new(),
            }
        }

        pub fn route(&mut self, path: &str, method: &str) {
            self.uri_infos.push(UriInfo {
                path: path.to_string().clone(),
                rule_name: path.to_string().clone(),
                service_name: None,
                method_name: method.to_string(),
            });
        }
    }

    impl IRouter for ShenYuRouter {
        fn app_name(&self) -> &str {
            &self.app_name
        }

        fn uri_infos(&self) -> &Vec<UriInfo> {
            &self.uri_infos
        }
    }

    /// Macro to register the `ShenYu` client once.
    ///
    /// This macro ensures that the `ShenYu` client is registered only once using a `OnceLock`.
    /// It initializes the client with the provided configuration, router, and port, and sets up
    /// a shutdown hook to deregister the client upon receiving a `ctrl_c` signal.
    ///
    /// # Arguments
    ///
    /// * `$config` - The configuration for the `ShenYu` client.
    /// * `$router` - The router instance.
    /// * `$port` - The port number.
    #[macro_export]
    macro_rules! register_once {
        ($config:expr, $router:expr, $port:literal) => {
            use std::sync::OnceLock;
            use $crate::IRouter;

            static ONCE: OnceLock<()> = OnceLock::new();
            ONCE.get_or_init(|| {
                let client = {
                    let res = $crate::core::ShenyuClient::new(
                        $config,
                        $router.app_name(),
                        $router.uri_infos(),
                        $port,
                    );
                    let client = res.unwrap();
                    client
                };
                client.register().expect("Failed to register");
                actix_web::rt::spawn(async move {
                    // Add shutdown hook
                    tokio::select! {
                        _ = actix_web::rt::signal::ctrl_c() => {
                            client.offline_register();
                        }
                    }
                });
            });
        };
    }

    /// Macro to define routes for the `ShenYu` router.
    ///
    /// This macro allows you to define routes for the `ShenYu` router in a concise manner.
    /// It supports both regular routes and nested routes.
    ///
    /// # Arguments
    ///
    /// * `$router` - The router instance.
    /// * `$app` - The Actix web application instance.
    /// * `$path` - The path for the route.
    /// * `$method` - The HTTP method for the route (e.g., `get`, `post`).
    /// * `$handler` - The handler function for the route.
    ///
    #[macro_export]
    macro_rules! shenyu_router {
        ($router:expr, $app:expr, $($path:expr => $method:ident($handler:expr))*) => {
            $(
                // fixme the handler method name
                $router.route($path, stringify!($method));
                $app = $app.service(actix_web::web::resource($path).route(actix_web::web::$method().to($handler)));
            )*
        }
    }
}

#[cfg(test)]
#[cfg(feature = "axum")]
mod tests_axum {
    use super::axum_impl::ShenYuRouter;
    use crate::config::ShenYuConfig;
    use crate::core::ShenyuClient;
    use crate::IRouter;
    use axum::routing::{get, post};
    use serde_json::Value;
    use std::collections::HashMap;

    async fn health_handler() -> &'static str {
        "OK"
    }

    async fn create_user_handler() -> &'static str {
        "User created"
    }

    #[tokio::test]
    async fn test_login() {
        let mut hashmap = HashMap::new();
        _ = hashmap.insert("username", "admin");
        _ = hashmap.insert("password", "123456");
        let params = [
            ("userName", hashmap.get("username").copied().unwrap()),
            ("password", hashmap.get("password").copied().unwrap()),
        ];

        // Fix the URL to include the scheme
        let res = ureq::get("http://127.0.0.1:9095/platform/login")
            .query_pairs(params)
            .call()
            .unwrap();
        let res_data: Value = res.into_json().unwrap();
        print!("res_data: {:?}", res_data);
        print!("res_data:token {:?}", res_data["data"]["token"]);
    }

    #[tokio::test]
    async fn build_client() {
        let app = ShenYuRouter::<()>::new("shenyu_client_app")
            .nest("/api", ShenYuRouter::new("api"))
            .route("/health", "get", get(health_handler))
            .route("/users", "post", post(create_user_handler));
        let config = ShenYuConfig::from_yaml_file("config.yml").unwrap();
        let res = ShenyuClient::new(config, app.app_name(), app.uri_infos(), 9527);
        assert!(&res.is_ok());
        let client = &mut res.unwrap();

        client.register().unwrap();
        client.offline_register();
    }

    #[test]
    fn it_works() {
        let binding = ShenYuRouter::<()>::new("shenyu_client_app");
        let app = binding
            .nest("/api", ShenYuRouter::new("api"))
            .route("/health", "get", get(health_handler))
            .route("/users", "post", post(create_user_handler));
        let uri_infos = app.uri_infos();
        assert_eq!(uri_infos.len(), 2);
        assert_eq!(uri_infos[0].path, "/health");
        assert_eq!(uri_infos[1].path, "/users");
    }
}

#[cfg(test)]
#[cfg(feature = "actix-web")]
mod tests_actix_web {
    use super::actix_web_impl::ShenYuRouter;
    use crate::config::ShenYuConfig;
    use crate::core::ShenyuClient;
    use crate::IRouter;

    #[tokio::test]
    async fn build_client() {
        let app = ShenYuRouter::new("shenyu_client_app");
        let config = ShenYuConfig::from_yaml_file("config.yml").unwrap();
        let res = ShenyuClient::new(config, app.app_name(), app.uri_infos(), 9527);
        assert!(&res.is_ok());
        let client = &mut res.unwrap();

        client.register().unwrap();
        client.offline_register();
    }
}
