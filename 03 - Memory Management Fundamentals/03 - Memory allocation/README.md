# Memory allocation

Allocation reserves storage with a size and alignment. Standard containers manage that storage and initialize elements as they are added. Capacity is reserved space and length counts initialized elements.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let mut scores = Vec::with_capacity(8);
    scores.extend([4, 9]);
    assert_eq!(scores.len(), 2);
    assert!(scores.capacity() >= 8);
}
```

## 2. Common mistake

Reserved capacity does not make extra indexes valid.

## Practice

Add items while printing capacity and identify when storage needs to grow.

## References

- [Official Rust documentation · ch04 01 what is ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

---

[← Previous: Heap memory](<../02 - Heap memory/README.md>) | [Back to Memory Management Fundamentals](<../README.md>) | [Next: Memory deallocation →](<../04 - Memory deallocation/README.md>)
