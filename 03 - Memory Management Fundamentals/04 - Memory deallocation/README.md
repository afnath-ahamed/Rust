# Memory deallocation

When an owning container is dropped, it drops its elements and releases its backing allocation as appropriate. Explicit drop can end a resource's use before the enclosing scope ends.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let buffer = vec![0u8; 128];
    assert_eq!(buffer.len(), 128);
    drop(buffer);
}
```

## 2. Common mistake

Clearing a Vec removes its elements and generally retains its capacity for reuse.

## Practice

Compare len and capacity before and after clear, then explain what dropping the vector does.

## References

- [Official Rust documentation · ch04 01 what is ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

---

[← Previous: Memory allocation](<../03 - Memory allocation/README.md>) | [Back to Memory Management Fundamentals](<../README.md>) | [Next: Pointers →](<../05 - Pointers/README.md>)
