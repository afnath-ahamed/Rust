# Collect

collect materializes an iterator into a destination implementing FromIterator. An annotation or explicit generic argument often selects the collection and can determine how fallible items are handled.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let values: Result<Vec<u16>, _> = ["3", "7"].into_iter().map(str::parse).collect();
    assert_eq!(values.unwrap(), [3, 7]);
}
```

## 2. Common mistake

Collecting a Result<Vec<_>, _> stops at the first error.

## Practice

Insert an invalid element and compare the result with collecting a Vec<Result<_, _>>.

## References

- [Official Rust documentation · ch13 00 functional features](https://doc.rust-lang.org/book/ch13-00-functional-features.html)

---

[← Previous: Fold](<../07 - Fold/README.md>) | [Back to Functional Programming](<../README.md>) | [Next: Lazy evaluation →](<../09 - Lazy evaluation/README.md>)
