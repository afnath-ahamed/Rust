# Atomics

Atomic operations can update a single shared value without a mutex. Relaxed ordering gives atomicity for operations on that atomic without ordering independent memory accesses.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let visits = AtomicUsize::new(0);
    visits.fetch_add(1, Ordering::Relaxed);
    assert_eq!(visits.load(Ordering::Relaxed), 1);
}
```

## 2. Common mistake

Publishing related data or maintaining multi-field invariants needs a stronger design than swapping ordering names until it compiles.

## Practice

Explain why a metrics counter can use Relaxed while related shared state may need a lock.

## References

- [Official Rust documentation · ch16 00 concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

---

[← Previous: Send and Sync traits](<../10 - Send and Sync traits/README.md>) | [Back to Concurrency and Parallelism](<../README.md>) | [Next: Race conditions →](<../12 - Race conditions/README.md>)
