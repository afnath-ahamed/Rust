# Move closures

A move closure captures its inputs by value. Moving an owned value into a spawned worker avoids borrowing local data that could disappear before the worker completes.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let job = String::from("work");
    let worker = std::thread::spawn(move || job.len());
    assert_eq!(worker.join().unwrap(), 4);
}
```

## 2. Common mistake

Moving a reference into the closure still leaves its original lifetime constraint.

## Practice

Return the owned String from the worker and use it after joining.

## Related lesson

[Closures](<../../12 - Functional Programming/01 - Closures/README.md>)

## References

- [Official Rust documentation · ch16 00 concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

---

[← Previous: Thread spawning](<../03 - Thread spawning/README.md>) | [Back to Concurrency and Parallelism](<../README.md>) | [Next: Message passing →](<../05 - Message passing/README.md>)
