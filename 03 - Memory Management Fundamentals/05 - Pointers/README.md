# Pointers

A pointer represents an address. Safe references additionally carry validity and borrowing guarantees, while raw pointer types place the responsibility for safe dereferencing on the programmer.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let reading = 7;
    let borrowed = &reading;
    let raw: *const i32 = std::ptr::from_ref(borrowed);
    assert!(!raw.is_null());
    assert_eq!(*borrowed, 7);
}
```

## 2. Common mistake

An address being non-null is insufficient evidence that it may be dereferenced.

## Practice

Explain which operation in the example reads the value and which only creates a pointer.

## References

- [Official Rust documentation · ch04 01 what is ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

---

[← Previous: Memory deallocation](<../04 - Memory deallocation/README.md>) | [Back to Memory Management Fundamentals](<../README.md>) | [Next: Garbage collection →](<../06 - Garbage collection/README.md>)
