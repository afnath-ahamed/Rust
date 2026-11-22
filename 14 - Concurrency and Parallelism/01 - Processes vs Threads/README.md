# Processes vs Threads

A process usually has its own address space. Threads within a process share its memory and have separate execution stacks. Use processes for isolation boundaries and threads for shared in-process work.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let worker = std::thread::spawn(std::process::id);
    assert_eq!(worker.join().unwrap(), std::process::id());
}
```

## 2. Common mistake

Thread failure and process failure have different isolation and recovery properties.

## Practice

Sketch a design with an isolated worker process and name what crosses the boundary.

## References

- [Official Rust documentation · ch16 00 concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

---

[← Previous: Smart Pointers](<../../13 - Smart Pointers/README.md>) | [Back to Concurrency and Parallelism](<../README.md>) | [Next: Multithreading →](<../02 - Multithreading/README.md>)
