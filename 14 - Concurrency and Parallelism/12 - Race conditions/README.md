# Race conditions

A race condition is a correctness bug dependent on interleaving. Safe Rust prevents data races subject to sound abstractions, but two individually safe operations can still break a check-then-act invariant.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let slots = std::sync::Mutex::new(1u32);
    let acquired = {
        let mut count = slots.lock().unwrap();
        if *count > 0 {
            *count -= 1;
            true
        } else {
            false
        }
    };
    assert!(acquired);
}
```

## 2. Common mistake

Make the check and update one logical operation.

## Practice

Draw an interleaving showing why using separate lock acquisitions for check and update can overbook a slot.

## References

- [Official Rust documentation · ch16 00 concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

---

[← Previous: Atomics](<../11 - Atomics/README.md>) | [Back to Concurrency and Parallelism](<../README.md>) | [Next: Deadlocks →](<../13 - Deadlocks/README.md>)
