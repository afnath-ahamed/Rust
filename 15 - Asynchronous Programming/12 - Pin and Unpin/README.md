# Pin and Unpin

Pin preserves address-sensitive guarantees by restricting moves through a pinned pointer. Unpin types do not impose additional move restrictions when pinned. Many compiler-generated futures need stable storage after polling.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    use std::future::Future;
    let future = async { 42 };
    let pinned = std::pin::pin!(future);
    fn accept(_: impl Future<Output = i32>) {}
    accept(pinned);
}
```

## 2. Common mistake

Pinning constrains the pointee through the pointer; the pointer handle can often be moved.

## Practice

Compare pin! for local storage with Box::pin for an owned heap-backed handle.

## References

- [Official Rust documentation · ch17 00 async await](https://doc.rust-lang.org/book/ch17-00-async-await.html)

---

[← Previous: Cancellation](<../11 - Cancellation/README.md>) | [Back to Asynchronous Programming](<../README.md>) | [Next: Async error handling →](<../13 - Async error handling/README.md>)
