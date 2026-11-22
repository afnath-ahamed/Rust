# Mutex

A Mutex allows one lock holder at a time. Rust ties access to a guard that releases the lock when dropped. A poisoned lock signals that a thread panicked while holding it.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let inventory = std::sync::Mutex::new(vec![1]);
    {
        inventory.lock().unwrap().push(2);
    }
    assert_eq!(inventory.lock().unwrap().as_slice(), &[1, 2]);
}
```

## 2. Common mistake

Recovering a poisoned lock requires deciding whether the protected state is still valid.

## Practice

Extract a snapshot under a lock and process it after dropping the guard.

## References

- [Official Rust documentation · ch16 00 concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

---

[← Previous: Shared state](<../07 - Shared state/README.md>) | [Back to Concurrency and Parallelism](<../README.md>) | [Next: RwLock →](<../09 - RwLock/README.md>)
