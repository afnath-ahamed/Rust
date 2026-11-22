# Associated types

An associated type selects an output type for each trait implementation. Iterator::Item is a common example. Generic bounds can constrain the chosen type.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn sum_bytes<I: Iterator<Item = u8>>(input: I) -> u32 {
    input.map(u32::from).sum()
}
assert_eq!(sum_bytes([1u8, 2, 3].into_iter()), 6);
```

## 2. Common mistake

A generic parameter can allow multiple implementations for a type where an associated choice typically describes one implementation relationship.

## Practice

Design a Decode trait with associated Output and Error types.

## References

- [Official Rust documentation · ch10 00 generics](https://doc.rust-lang.org/book/ch10-00-generics.html)

---

[← Previous: Default implementations](<../07 - Default implementations/README.md>) | [Back to Generics, Traits and Lifetimes](<../README.md>) | [Next: Lifetime annotations →](<../09 - Lifetime annotations/README.md>)
