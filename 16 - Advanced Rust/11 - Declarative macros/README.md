# Declarative macros

macro_rules! macros expand matching token patterns into Rust syntax. Use the right fragment kinds and let a normal function handle work that does not require syntax generation.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
macro_rules! doubled {
    ($value:expr) => {{
        let value = $value;
        value + value
    }};
}
assert_eq!(doubled!(3 + 4), 14);
```

## 2. Common mistake

Expanding an argument multiple times can duplicate side effects. The example binds it once.

## Practice

Add a repetition arm to build a vector from a list of expressions.

## References

- [Official Rust documentation · ch20 00 advanced features](https://doc.rust-lang.org/book/ch20-00-advanced-features.html)

---

[← Previous: Generic associated types](<../10 - Generic associated types/README.md>) | [Back to Advanced Rust](<../README.md>) | [Next: Procedural macros →](<../12 - Procedural macros/README.md>)
