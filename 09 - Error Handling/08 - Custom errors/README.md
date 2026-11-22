# Custom errors

A typed error enumerates failure cases that callers may branch on. Implement Display for a readable message and Error for integration with the standard error ecosystem. Preserve underlying causes when they matter.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
#[derive(Debug, PartialEq)]
struct EmptyTitle;
impl std::fmt::Display for EmptyTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("title is empty")
    }
}
impl std::error::Error for EmptyTitle {}
fn title(input: &str) -> Result<&str, EmptyTitle> {
    if input.trim().is_empty() {
        Err(EmptyTitle)
    } else {
        Ok(input.trim())
    }
}
assert_eq!(title(" "), Err(EmptyTitle));
```

## 2. Common mistake

A single string error makes programmatic distinction brittle.

## Practice

Convert EmptyTitle into an enum with a TooLong variant and test both cases.

## References

- [Official Rust documentation · ch09 00 error handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

---

[← Previous: Error propagation (?)](<../07 - Error propagation/README.md>) | [Back to Error Handling](<../README.md>) | [Next: Thiserror →](<../09 - Thiserror/README.md>)
