# Foreign Function Interface (FFI)

FFI crosses an ABI and type boundary. Match the ABI and representation explicitly, communicate allocation ownership, and validate contract assumptions before wrapping a foreign call in safe Rust.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```rust
#![crate_type = "lib"]
#[repr(C)]
pub struct Pair {
    pub x: i32,
    pub y: i32,
}
pub extern "C" fn multiply(pair: Pair) -> i64 {
    i64::from(pair.x) * i64::from(pair.y)
}
```

## 2. Common mistake

Rust's default struct representation is unsuitable for a promised C layout. Export details and panic boundaries also need deliberate choices.

## Practice

Write a matching C declaration using fixed-width integers and identify who owns any future pointer argument.

## References

- [Official Rust documentation · ffi](https://doc.rust-lang.org/nomicon/ffi.html)
- [Official Rust documentation · ch20 00 advanced features](https://doc.rust-lang.org/book/ch20-00-advanced-features.html)

---

[← Previous: Procedural macros](<../12 - Procedural macros/README.md>) | [Back to Advanced Rust](<../README.md>) | [Next: no_std Rust →](<../14 - no_std Rust/README.md>)
