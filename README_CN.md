# Apache ShenYu-Client-Rust ShenYu-Rust客户端

Apache ShenYu Rust客户端SDK是一个用于与Apache ShenYu网关交互的Rust库。此SDK允许您轻松地将Rust应用程序与ShenYu网关集成，提供一种无缝的方式来管理和路由API请求。

## 安装

要在项目中使用Apache ShenYu Rust客户端SDK，请在`Cargo.toml`文件中添加以下依赖项：

```toml
[dependencies]
serde = "1.0.190"
serde_json = "1.0.80"
reqwest = "0.12.5"
axum = "0.5"
tokio = "1.39.3"
shenyu-client-rust = {version = "0.1.1", features = ["actix-web", "axum"] }
```

## 使用

下面是一个如何使用`ShenYuRouter`创建Axum服务并将其与ShenYu网关集成的示例。

### 示例

```rust

#![cfg(feature = "axum")]
use axum::routing::post;
use axum::{routing::get, Router};
use shenyu_client_rust::axum_impl::ShenYuRouter;
use shenyu_client_rust::config::ShenYuConfig;
use shenyu_client_rust::{core::ShenyuClient, IRouter};
use tokio::signal;

async fn health_handler() -> &'static str {
    "OK"
}

async fn create_user_handler() -> &'static str {
    "User created"
}

#[tokio::main]
async fn main() {
    let app = ShenYuRouter::<()>::new("shenyu_client_app")
        .nest("/api", ShenYuRouter::new("api"))
        .route("/health", get(health_handler))
        .route("/users", post(create_user_handler));
    let config = ShenYuConfig::from_yaml_file("examples/config.yml").unwrap();
    let client = ShenyuClient::from(config, app.app_name(), app.uri_infos(), 3000)
        .await
        .unwrap();

    let axum_app: Router = app.into();
    client.register().await.expect("TODO: panic message");

    // Start Axum server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, axum_app)
        .with_graceful_shutdown(async move {
            signal::ctrl_c().await.expect("failed to listen for event");
            client.offline_register().await;
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
use shenyu_client_rust::{core::ShenyuClient, register_once, shenyu_router, IRouter};

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

此示例演示了如何使用`ShenYuRouter`设置基本的Axum服务并将其注册到ShenYu网关。`health_handler`和`create_user_handler`是处理HTTP请求的简单异步函数。

## 许可证

此项目根据Apache许可证2.0版获得许可。有关更多详细信息，请参阅[LICENSE](LICENSE)文件。