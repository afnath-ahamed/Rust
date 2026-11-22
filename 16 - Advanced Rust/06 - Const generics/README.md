# Const generics

Const generics parameterize types or functions by supported constant values such as lengths. They preserve fixed dimensions in type signatures.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn copy_array<const N: usize>(input: [u8; N]) -> [u8; N] {
    input
}
assert_eq!(copy_array([1, 2, 3]), [1, 2, 3]);
```

## 2. Common mistake

Simple const arguments are portable stable Rust. More complex arithmetic with generic consts has additional restrictions.

## Practice

Wrap an array in a const-generic type and add a length method.

## References

- [Official Rust documentation · ch20 00 advanced features](https://doc.rust-lang.org/book/ch20-00-advanced-features.html)

---

[← Previous: Associated constants](<../05 - Associated constants/README.md>) | [Back to Advanced Rust](<../README.md>) | [Next: Type aliases →](<../07 - Type aliases/README.md>)
