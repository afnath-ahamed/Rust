# Thiserror

The thiserror crate derives standard Error implementations for typed errors. Use it when callers need named variants or a source chain without writing repetitive formatting code.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
cargo add thiserror
```

```rust no_run
#[derive(Debug, thiserror::Error)]
enum LoadError {
    #[error("could not read settings")]
    Read(#[from] std::io::Error),
}
fn load(path: &std::path::Path) -> Result<String, LoadError> {
    Ok(std::fs::read_to_string(path)?)
}
```

## 2. Common mistake

Avoid exposing raw low-level errors as a public contract when callers need stable domain variants.

## Practice

Add a validation variant and assert that a missing file becomes the Read variant.

## References

- [docs.rs · thiserror](https://docs.rs/thiserror/latest/thiserror/)
- [Official Rust documentation · ch09 00 error handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

---

[← Previous: Custom errors](<../08 - Custom errors/README.md>) | [Back to Error Handling](<../README.md>) | [Next: Anyhow →](<../10 - Anyhow/README.md>)
