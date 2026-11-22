# Futures

Future::poll takes a pinned receiver and context. It returns Ready with a result or Pending after arranging an appropriate wake-up when progress becomes possible.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    use std::future::{Future, ready};
    use std::task::{Context, Poll, Waker};
    let mut future = std::pin::pin!(ready(42));
    let mut context = Context::from_waker(Waker::noop());
    assert_eq!(future.as_mut().poll(&mut context), Poll::Ready(42));
}
```

## 2. Common mistake

A pending future needs a way to wake its task. Continuously polling it wastes CPU.

## Practice

Explain what a timer stores while it waits and why the executor can sleep.

## References

- [Official Rust documentation · ch17 00 async await](https://doc.rust-lang.org/book/ch17-00-async-await.html)

---

[← Previous: Async / Await](<../03 - Async and Await/README.md>) | [Back to Asynchronous Programming](<../README.md>) | [Next: Executors →](<../05 - Executors/README.md>)
