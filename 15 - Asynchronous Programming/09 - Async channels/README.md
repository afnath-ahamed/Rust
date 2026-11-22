# Async channels

A bounded async channel suspends a producer when capacity is exhausted. The send and receive sides can observe closure. Use capacity to limit buffered work and memory.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```rust no_run
async fn transfer() -> Option<u32> {
    let (sender, mut receiver) = tokio::sync::mpsc::channel(1);
    sender.send(7).await.ok()?;
    receiver.recv().await
}
```

## 2. Common mistake

Enable Tokio's sync feature. A full queue cannot drain if its only consumer is waiting for the producer to finish first.

## Practice

Design a shutdown sequence that stops submissions and drains accepted jobs.

## References

- [Official Rust documentation · ch17 00 async await](https://doc.rust-lang.org/book/ch17-00-async-await.html)

---

[← Previous: Async tasks](<../08 - Async tasks/README.md>) | [Back to Asynchronous Programming](<../README.md>) | [Next: Streams →](<../10 - Streams/README.md>)
