# Tracing

Tracing uses spans and events to connect operations across nested and asynchronous work. Attach stable identifiers and operation context to a span, then correlate output by the same context.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
cargo add tracing
```

```rust no_run
fn main() {
    let span = tracing::info_span!("import", file_count = 3);
    span.in_scope(|| tracing::info!(rows = 12, "finished"));
}
```

## 2. Common mistake

A synchronous span guard held across await can misattribute interleaved work. Use async-aware instrumentation.

## Practice

Instrument a parent operation and a child stage and validate the resulting context.

## References

- [docs.rs · tracing](https://docs.rs/tracing/latest/tracing/)
- [Official Rust documentation · ch11 00 testing](https://doc.rust-lang.org/book/ch11-00-testing.html)

---

[← Previous: Logging](<../09 - Logging/README.md>) | [Back to Testing and Debugging](<../README.md>) | [Next: Code coverage →](<../11 - Code coverage/README.md>)
