# Actix Web

Actix Web organizes services around an App built by HttpServer. Use extractors for request data and shared application state for resources such as a pool.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
cargo add actix-web
```

```rust no_run
use actix_web::{App, HttpServer, get};
#[get("/health")]
async fn health() -> &'static str {
    "ok"
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
```

## 2. Common mistake

The server factory can be called for multiple workers. Create intentionally shared resources outside it and clone their handles into each application.

## Practice

Add a JSON handler and test it through Actix Web's local test utilities.

## References

- [docs.rs · axum](https://docs.rs/axum/latest/axum/)

---

[← Previous: Axum](<../03 - Axum/README.md>) | [Back to Backend and Web Development](<../README.md>) | [Next: Routing →](<../05 - Routing/README.md>)
