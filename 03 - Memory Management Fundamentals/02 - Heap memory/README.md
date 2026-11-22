# Heap memory

Heap storage supports values whose allocated storage can outlive the scope that produced them when ownership is transferred. A Box owns a heap allocation for a nonzero-sized value, and a Vec usually owns a separately allocated element buffer.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let values = Box::new([2, 4, 8]);
    assert_eq!(values[2], 8);
}
```

## 2. Common mistake

A heap-backed data structure can still have a local handle whose storage is chosen by the compiler.

## Practice

Compare the length of a Vec and the number of elements it can hold before reallocating.

## References

- [Official Rust documentation · ch04 01 what is ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

---

[← Previous: Stack memory](<../01 - Stack memory/README.md>) | [Back to Memory Management Fundamentals](<../README.md>) | [Next: Memory allocation →](<../03 - Memory allocation/README.md>)
