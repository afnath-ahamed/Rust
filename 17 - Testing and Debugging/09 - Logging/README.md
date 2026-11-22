# Logging

Logging records events along with appropriate severity and context. Use structured fields when the reader needs to filter events, and avoid sensitive values and high-volume noise.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
cargo add log
```

```rust no_run
log::info!("processed {} records", 12);
```

## 2. Common mistake

The log facade requires an initialized logger to emit messages.

## Practice

Configure a logger for your binary and demonstrate changing the level at runtime.

## References

- [Official Rust documentation · ch11 00 testing](https://doc.rust-lang.org/book/ch11-00-testing.html)

---

[← Previous: Debugging](<../08 - Debugging/README.md>) | [Back to Testing and Debugging](<../README.md>) | [Next: Tracing →](<../10 - Tracing/README.md>)
