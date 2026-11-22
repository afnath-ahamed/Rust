# Blocking vs Non-blocking

A blocking operation can prevent its current thread from progressing. On a runtime worker, that also delays unrelated tasks scheduled on the worker. Use async I/O or an appropriate dedicated blocking pool.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
cargo add tokio --features macros,rt-multi-thread
```

```rust no_run
async fn report() -> Result<usize, tokio::task::JoinError> {
    tokio::task::spawn_blocking(|| (1..=1000).sum::<usize>()).await
}
```

## 2. Common mistake

Moving CPU work into an async block does not turn it into non-blocking work.

## Practice

Compare timer responsiveness with heavy synchronous work on a runtime worker and in the blocking pool.

## References

- [Official Rust documentation · ch17 00 async await](https://doc.rust-lang.org/book/ch17-00-async-await.html)

---

[← Previous: Synchronous vs Asynchronous](<../01 - Synchronous vs Asynchronous/README.md>) | [Back to Asynchronous Programming](<../README.md>) | [Next: Async / Await →](<../03 - Async and Await/README.md>)
