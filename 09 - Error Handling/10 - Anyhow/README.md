# Anyhow

anyhow provides a convenient application error type and context chaining. It works well for top-level operations that report failures. Keep typed errors where downstream code needs stable matching.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
cargo add anyhow
```

```rust no_run
use anyhow::Context;
fn load() -> anyhow::Result<String> {
    std::fs::read_to_string("settings.toml").context("loading the application settings")
}
```

## 2. Common mistake

Do not attach sensitive configuration values to an error report.

## Practice

Run the function with a missing file and verify that both the operation and original cause are visible.

## References

- [docs.rs · anyhow](https://docs.rs/anyhow/latest/anyhow/)
- [Official Rust documentation · ch09 00 error handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

---

[← Previous: Thiserror](<../09 - Thiserror/README.md>) | [Back to Error Handling](<../README.md>) | [Next: Project Organization →](<../../10 - Project Organization/README.md>)
