# Memory safety

Memory safety rules prevent invalid access patterns such as reading uninitialized or freed data through safe abstractions. Checked APIs can turn invalid indexes into absence rather than panicking.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let samples = [8, 13, 21];
    assert_eq!(samples.get(1), Some(&13));
    assert_eq!(samples.get(99), None);
}
```

## 2. Common mistake

Memory safety leaves room for incorrect answers, resource exhaustion, and application-level bugs.

## Practice

Replace direct indexing in a small example with get and test an invalid index.

## References

- [Official Rust documentation · ch04 01 what is ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

---

[← Previous: Memory leaks](<../08 - Memory leaks/README.md>) | [Back to Memory Management Fundamentals](<../README.md>) | [Next: Ownership System →](<../../04 - Ownership System/README.md>)
