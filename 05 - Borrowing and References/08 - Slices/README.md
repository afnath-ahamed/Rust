# Slices

A slice is a borrowed view of a contiguous sequence. &[T] and &mut [T] carry a length. A &str also has a byte length and must remain valid UTF-8.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let readings = [5, 8, 13, 21];
    let middle: &[i32] = &readings[1..3];
    assert_eq!(middle, [8, 13]);
    assert_eq!("é".get(..1), None);
}
```

## 2. Common mistake

String byte ranges must land on character boundaries and be within the string length.

## Practice

Write a function returning the first two elements using get on a range.

## References

- [Official Rust documentation · ch04 02 references and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

---

[← Previous: Dereferencing](<../07 - Dereferencing/README.md>) | [Back to Borrowing and References](<../README.md>) | [Next: Structs and Enums →](<../../06 - Structs and Enums/README.md>)
