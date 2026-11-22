# Unsafe Rust

Unsafe Rust permits specific operations whose safety depends on obligations the compiler cannot establish. Keep the unsafe block small and explain each invariant at the operation. Prefer a checked safe API where it expresses the same contract.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let data = [3, 8, 13];
    assert_eq!(data.get(1), Some(&8));
}
```

## 2. Common mistake

An unsafe block does not switch off the borrow checker. It permits specific operations such as raw dereferencing.

## Practice

Write the preconditions an unchecked access would require, then keep the production version checked.

## References

- [Official Rust documentation · ch20 00 advanced features](https://doc.rust-lang.org/book/ch20-00-advanced-features.html)

---

[← Previous: Asynchronous Programming](<../../15 - Asynchronous Programming/README.md>) | [Back to Advanced Rust](<../README.md>) | [Next: Raw pointers →](<../02 - Raw pointers/README.md>)
