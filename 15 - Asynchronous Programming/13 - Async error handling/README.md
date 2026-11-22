# Async error handling

Async functions can return Result like synchronous functions. Separate operation errors, timeout errors, and task failures when each needs different handling.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    async fn parse(text: &str) -> Result<u16, std::num::ParseIntError> {
        Ok(text.parse::<u16>()?)
    }
    let _future = parse("42");
}
```

## 2. Common mistake

Joining a spawned Result-returning task can produce a nested Result. The outer failure belongs to the task boundary.

## Practice

Write down how your caller would handle invalid input, cancellation, and a service timeout separately.

## References

- [Official Rust documentation · ch17 00 async await](https://doc.rust-lang.org/book/ch17-00-async-await.html)

---

[← Previous: Pin and Unpin](<../12 - Pin and Unpin/README.md>) | [Back to Asynchronous Programming](<../README.md>) | [Next: Advanced Rust →](<../../16 - Advanced Rust/README.md>)
