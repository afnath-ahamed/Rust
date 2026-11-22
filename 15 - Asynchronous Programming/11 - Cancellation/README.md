# Cancellation

Cancellation often drops a future. A select branch losing a race may lose progress if that operation is not cancellation safe. Use shutdown notifications and invariants for partial effects.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```rust no_run
async fn wait() -> bool {
    tokio::time::timeout(
        std::time::Duration::from_millis(5),
        std::future::pending::<()>(),
    )
    .await
    .is_ok()
}
```

## 2. Common mistake

A timeout limits waiting but does not undo an external effect that already happened.

## Practice

For a timed-out write, design a request identifier and a safe way to determine whether it completed.

## Related lesson

[Channels](<../../14 - Concurrency and Parallelism/06 - Channels/README.md>)

## References

- [Official Rust documentation · ch17 00 async await](https://doc.rust-lang.org/book/ch17-00-async-await.html)

---

[← Previous: Streams](<../10 - Streams/README.md>) | [Back to Asynchronous Programming](<../README.md>) | [Next: Pin and Unpin →](<../12 - Pin and Unpin/README.md>)
