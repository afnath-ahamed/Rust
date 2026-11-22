# Synchronous vs Asynchronous

Synchronous code completes each call in its current control flow. Async code can suspend and resume at await points when its future is driven by an executor. Use ordinary synchronous code for straightforward isolated work.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    use std::future::Future;
    fn synchronous() -> u32 {
        7
    }
    async fn asynchronous() -> u32 {
        synchronous()
    }
    fn assert_future(_: impl Future<Output = u32>) {}
    assert_future(asynchronous());
}
```

## 2. Common mistake

Calling an async function creates a future. Its body needs to be polled before progress occurs.

## Practice

Identify which operations of a small read-transform-write command could overlap.

## References

- [Official Rust documentation · ch17 00 async await](https://doc.rust-lang.org/book/ch17-00-async-await.html)

---

[← Previous: Concurrency and Parallelism](<../../14 - Concurrency and Parallelism/README.md>) | [Back to Asynchronous Programming](<../README.md>) | [Next: Blocking vs Non-blocking →](<../02 - Blocking vs Non-blocking/README.md>)
