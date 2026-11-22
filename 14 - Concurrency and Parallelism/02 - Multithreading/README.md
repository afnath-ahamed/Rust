# Multithreading

Concurrency means managing multiple tasks in overlapping time. Parallelism means executing work simultaneously. Splitting independent CPU work can use multiple cores when scheduling and data permit.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let left = std::thread::spawn(|| (1..=3).sum::<u32>());
    let right = std::thread::spawn(|| (4..=6).sum::<u32>());
    assert_eq!(left.join().unwrap() + right.join().unwrap(), 21);
}
```

## 2. Common mistake

Spawning a thread per tiny operation may cost more than doing each operation inline.

## Practice

Benchmark sequential summation and chunked summation for multiple input sizes.

## References

- [Official Rust documentation · ch16 00 concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

---

[← Previous: Processes vs Threads](<../01 - Processes vs Threads/README.md>) | [Back to Concurrency and Parallelism](<../README.md>) | [Next: Thread spawning →](<../03 - Thread spawning/README.md>)
