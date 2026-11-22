# Streams

A Stream is an asynchronous sequence. Consumers read items as they become available, often with StreamExt. Cap concurrency and buffering when downstream work is slower.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
cargo add futures
```

```rust no_run
use futures::StreamExt;
async fn sizes() -> Vec<usize> {
    futures::stream::iter(["a", "rust"])
        .map(str::len)
        .collect()
        .await
}
```

## 2. Common mistake

Collect only finite or deliberately bounded streams.

## Practice

Consume two items and explain how early termination affects remaining work.

## References

- [Official Rust documentation · ch17 00 async await](https://doc.rust-lang.org/book/ch17-00-async-await.html)

---

[← Previous: Async channels](<../09 - Async channels/README.md>) | [Back to Asynchronous Programming](<../README.md>) | [Next: Cancellation →](<../11 - Cancellation/README.md>)
