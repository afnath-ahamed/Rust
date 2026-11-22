# RwLock

An RwLock allows many readers or one writer under its locking policy. Consider it for heavily read shared data, and benchmark against Mutex because contention and platform behavior matter.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let setting = std::sync::RwLock::new(7);
    assert_eq!(*setting.read().unwrap(), 7);
    *setting.write().unwrap() = 9;
    assert_eq!(*setting.read().unwrap(), 9);
}
```

## 2. Common mistake

Avoid holding a read lock while trying to acquire a write lock on the same object.

## Practice

Build a configuration snapshot read by several scoped threads.

## References

- [Official Rust documentation · ch16 00 concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

---

[← Previous: Mutex](<../08 - Mutex/README.md>) | [Back to Concurrency and Parallelism](<../README.md>) | [Next: Send and Sync traits →](<../10 - Send and Sync traits/README.md>)
