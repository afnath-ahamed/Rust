# Async tasks

A spawned task makes progress independently. Its JoinHandle provides completion and failure information. Tokio's multi-threaded spawn generally requires Send and 'static for the future and its result.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```rust no_run
async fn compute() -> Result<u32, tokio::task::JoinError> {
    let handle = tokio::spawn(async move { 6 * 7 });
    handle.await
}
```

## 2. Common mistake

Dropping a Tokio JoinHandle detaches the task. Track work that must report failures or terminate on shutdown.

## Practice

Spawn two owned jobs and handle each join result.

## References

- [Official Rust documentation · ch17 00 async await](https://doc.rust-lang.org/book/ch17-00-async-await.html)

---

[← Previous: Tokio](<../07 - Tokio/README.md>) | [Back to Asynchronous Programming](<../README.md>) | [Next: Async channels →](<../09 - Async channels/README.md>)
