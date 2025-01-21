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

See [examples](https://github.com/apache/shenyu-client-rust/tree/main/examples).

This example demonstrates how to set up a basic Axum service using `ShenYuRouter` and register it with the `ShenYu` Gateway. `health_handler` and `create_user_handler` are simple asynchronous functions that handle HTTP requests.

## License

This project is licensed under the Apache License 2.0. For more details, see the [LICENSE](LICENSE) file.
