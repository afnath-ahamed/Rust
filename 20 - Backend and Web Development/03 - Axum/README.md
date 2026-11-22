# Axum

Axum builds services from routers, async handlers, and extractors. Put reusable application resources in state, parse input with extractors, and map domain results into HTTP responses.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
cargo add axum
cargo add tokio --features macros,rt-multi-thread,net
```

```rust no_run
use axum::{Router, routing::get};
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new().route("/health", get(|| async { "ok" }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await
}
```

## 2. Common mistake

Do not hold blocking locks or execute expensive synchronous work on async workers.

## Practice

Add a typed path extractor and return 404 for an unknown identifier.

## References

- [docs.rs · axum](https://docs.rs/axum/latest/axum/)

---

[← Previous: REST APIs](<../02 - REST APIs/README.md>) | [Back to Backend and Web Development](<../README.md>) | [Next: Actix Web →](<../04 - Actix Web/README.md>)
