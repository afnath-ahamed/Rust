# Associated constants

An associated constant is a value tied to a type or implementation. It is useful for compile-time settings that belong with behavior rather than per-instance state.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
trait Packet {
    const HEADER_BYTES: usize;
}
struct Small;
impl Packet for Small {
    const HEADER_BYTES: usize = 4;
}
assert_eq!(Small::HEADER_BYTES, 4);
```

## 2. Common mistake

Adding an associated constant to a trait affects whether it can be used as a trait object.

## Practice

Implement a second packet type with a different header size.

## References

- [Official Rust documentation · ch20 00 advanced features](https://doc.rust-lang.org/book/ch20-00-advanced-features.html)

---

[← Previous: Advanced traits](<../04 - Advanced traits/README.md>) | [Back to Advanced Rust](<../README.md>) | [Next: Const generics →](<../06 - Const generics/README.md>)
