# Thread spawning

spawn starts a thread and returns a JoinHandle. join waits for completion and can report a panic. Scoped threads can borrow data that remains valid until their scope ends.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let payload = [1, 2, 3];
    let sum =
        std::thread::scope(|scope| scope.spawn(|| payload.iter().sum::<i32>()).join().unwrap());
    assert_eq!(sum, 6);
}
```

## 2. Common mistake

Dropping a JoinHandle detaches the thread. It does not cancel it.

## Practice

Replace a detached worker with one whose failure is observed by its caller.

## References

- [Official Rust documentation · ch16 00 concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

---

[← Previous: Multithreading](<../02 - Multithreading/README.md>) | [Back to Concurrency and Parallelism](<../README.md>) | [Next: Move closures →](<../04 - Move closures/README.md>)
