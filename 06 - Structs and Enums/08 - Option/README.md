# `Option<T>`

Option<T> represents presence with Some(T) and absence with None. The same type represents both branches, so callers can choose how to handle absence explicitly.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let values = [4, 8];
    let selected = values.get(1).copied();
    assert_eq!(selected, Some(8));
    assert_eq!(values.get(4), None);
}
```

## 2. Common mistake

A sentinel like zero cannot represent absence if zero is also valid data.

## Practice

Implement a function that returns the first even number or None.

## Related lesson

[Option](<../../09 - Error Handling/05 - Option/README.md>)

## References

- [Official Rust documentation · ch05 00 structs](https://doc.rust-lang.org/book/ch05-00-structs.html)

---

[← Previous: Enums](<../07 - Enums/README.md>) | [Back to Structs and Enums](<../README.md>) | [Next: `Result<T, E>` →](<../09 - Result/README.md>)
