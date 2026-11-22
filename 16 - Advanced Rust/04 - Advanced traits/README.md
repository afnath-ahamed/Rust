# Advanced traits

Traits can share operations through default methods, associated items, supertraits, or generic constraints. Supertraits let a contract depend on an existing capability.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
trait Named: std::fmt::Display {
    fn category(&self) -> &'static str {
        "label"
    }
}
impl Named for String {}
assert_eq!(String::from("x").category(), "label");
```

## 2. Common mistake

Public trait design should consider which additions would require changes from downstream implementers.

## Practice

Add a default method that uses Display and explain why it compiles.

## References

- [Official Rust documentation · ch20 00 advanced features](https://doc.rust-lang.org/book/ch20-00-advanced-features.html)

---

[← Previous: Unsafe functions](<../03 - Unsafe functions/README.md>) | [Back to Advanced Rust](<../README.md>) | [Next: Associated constants →](<../05 - Associated constants/README.md>)
