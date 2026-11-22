# Tuple structs

Tuple structs create distinct named types with positional fields. A one-field tuple struct is a newtype, useful for giving a unit or domain identifier its own type.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
struct UserId(u64);
struct OrderId(u64);
fn find(id: UserId) -> u64 {
    id.0
}
assert_eq!(find(UserId(7)), 7);
assert_eq!(OrderId(9).0, 9);
```

## 2. Common mistake

Identical underlying field types do not make UserId and OrderId interchangeable.

## Practice

Wrap meters and seconds separately and restrict one function to meters.

## References

- [Official Rust documentation · ch05 00 structs](https://doc.rust-lang.org/book/ch05-00-structs.html)

---

[← Previous: Structs](<../01 - Structs/README.md>) | [Back to Structs and Enums](<../README.md>) | [Next: Unit-like structs →](<../03 - Unit-like structs/README.md>)
