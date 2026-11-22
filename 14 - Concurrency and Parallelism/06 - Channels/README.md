# Channels

A channel separates producers from a consumer. A bounded channel limits queued items and provides backpressure. Dropping all senders lets the receiver detect disconnection after draining.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let (sender, receiver) = std::sync::mpsc::sync_channel(1);
    sender.send(7).unwrap();
    assert_eq!(receiver.recv().unwrap(), 7);
    drop(sender);
    assert!(receiver.recv().is_err());
}
```

## 2. Common mistake

Waiting for completion while a producer blocks on a full channel can deadlock.

## Practice

Use try_send to demonstrate how a full queue is reported.

## References

- [Official Rust documentation · ch16 00 concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

---

[← Previous: Message passing](<../05 - Message passing/README.md>) | [Back to Concurrency and Parallelism](<../README.md>) | [Next: Shared state →](<../07 - Shared state/README.md>)
