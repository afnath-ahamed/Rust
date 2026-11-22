# Shared state

Shared state requires a policy for ownership and simultaneous access. Arc shares ownership and a synchronization primitive controls mutation when multiple threads need it.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let count = std::sync::Arc::new(std::sync::Mutex::new(0));
    let worker_count = count.clone();
    std::thread::spawn(move || *worker_count.lock().unwrap() += 1)
        .join()
        .unwrap();
    assert_eq!(*count.lock().unwrap(), 1);
}
```

## 2. Common mistake

Hold locks only while accessing the protected state. Move network or file work outside the critical section.

## Practice

Replace the single lock with message passing and compare the ownership model.

## References

- [Official Rust documentation · ch16 00 concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

---

[← Previous: Channels](<../06 - Channels/README.md>) | [Back to Concurrency and Parallelism](<../README.md>) | [Next: Mutex →](<../08 - Mutex/README.md>)
