# Apache ShenYu-Client-Rust

The Apache `ShenYu` Rust Client SDK is a Rust library for interacting with the Apache `ShenYu` gateway. This SDK allows you to easily integrate your Rust applications with the `ShenYu` gateway, providing a seamless way to manage and route your API requests.

## Installation

To use the Apache `ShenYu` Rust Client SDK in your project, add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
serde = "1.0.190"
serde_json = "1.0.80"
reqwest = "0.12.5"
axum = "0.5"
tokio = "1.39.3"
shenyu-client-rust = {version = "0.1.1", features = ["actix-web", "axum"] }
```

## Usage

Below is an example of how to create an Axum service using `ShenYuRouter` and integrate it with the `ShenYu` Gateway.

### Example

```rust

#![cfg(feature = "axum")]
use axum::routing::post;
use axum::{routing::get, Router};
use shenyu_client_rust::axum_impl::ShenYuRouter;
use shenyu_client_rust::config::ShenYuConfig;
use shenyu_client_rust::{core::ShenyuClient, IRouter};

mod ci;
use crate::ci::_CI_CTRL_C;

async fn health_handler() -> &'static str {
    "OK"
}

async fn create_user_handler() -> &'static str {
    "User created"
}

#[tokio::main]
async fn main() {
    // Spawn a thread to listen for Ctrl-C events and shutdown the server
    std::thread::spawn(_CI_CTRL_C);
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let app = ShenYuRouter::<()>::new("shenyu_client_app")
        .nest("/api", ShenYuRouter::new("api"))
        .route("/health", "get", get(health_handler))
        .route("/users", "post", post(create_user_handler));
    let config = ShenYuConfig::from_yaml_file("examples/config.yml").unwrap();
    let client = ShenyuClient::from(config, app.app_name(), app.uri_infos(), 3000).unwrap();

    let axum_app: Router = app.into();
    client.register().expect("TODO: panic message");

    // Start Axum server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, axum_app)
        .with_graceful_shutdown(async move {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to listen for event");
            client.offline_register();
        })
        .await
        .unwrap();
}

```
```rust
#![cfg(feature = "actix-web")]
use actix_web::{middleware, App, HttpServer, Responder};
use shenyu_client_rust::actix_web_impl::ShenYuRouter;
use shenyu_client_rust::config::ShenYuConfig;
use shenyu_client_rust::{register_once, shenyu_router};

mod ci;
use crate::ci::_CI_CTRL_C;

async fn health_handler() -> impl Responder {
    "OK"
}

async fn create_user_handler() -> impl Responder {
    "User created"
}

async fn index() -> impl Responder {
    "Welcome!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Spawn a thread to listen for Ctrl-C events and shutdown the server
    std::thread::spawn(_CI_CTRL_C);
    // Initialize tracing
    tracing_subscriber::fmt::init();

    HttpServer::new(move || {
        let mut router = ShenYuRouter::new("shenyu_client_app");
        let mut app = App::new().wrap(middleware::Logger::default());
        let config = ShenYuConfig::from_yaml_file("examples/config.yml").unwrap();
        shenyu_router!(
            router,
            app,
            "/health" => get(health_handler)
            "/create_user" => post(create_user_handler)
            "/" => get(index)
        );
        register_once!(config, router, 4000);

        app
    })
        .bind(("0.0.0.0", 4000))
        .expect("Can not bind to 4000")
        .run()
        .await
}

```

This example demonstrates how to set up a basic Axum service using `ShenYuRouter` and register it with the `ShenYu` Gateway. `health_handler` and `create_user_handler` are simple asynchronous functions that handle HTTP requests.

## License

This project is licensed under the Apache License 2.0. For more details, see the [LICENSE](LICENSE) file.
