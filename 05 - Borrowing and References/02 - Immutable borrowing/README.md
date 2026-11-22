# Immutable borrowing

A shared borrow allows multiple readers of the same value. Shared references do not generally give permission to mutate the referenced value. Interior-mutability types provide carefully checked exceptions.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let numbers = [2, 7, 8];
    let left = &numbers;
    let right = &numbers;
    assert_eq!(left[0] + right[2], 10);
}
```

## 2. Common mistake

An active shared borrow prevents another reference from taking ordinary exclusive access at the same time.

## Practice

Write two functions that borrow a String and call both without cloning.

## References

- [Official Rust documentation · ch04 02 references and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

---

[← Previous: References](<../01 - References/README.md>) | [Back to Borrowing and References](<../README.md>) | [Next: Mutable borrowing →](<../03 - Mutable borrowing/README.md>)
