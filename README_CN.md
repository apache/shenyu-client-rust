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

见 [examples](https://github.com/apache/shenyu-client-rust/tree/main/examples).

此示例演示了如何使用`ShenYuRouter`设置基本的Axum服务并将其注册到ShenYu网关。`health_handler`和`create_user_handler`是处理HTTP请求的简单异步函数。

## 许可证

此项目根据Apache许可证2.0版获得许可。有关更多详细信息，请参阅[LICENSE](LICENSE)文件。
