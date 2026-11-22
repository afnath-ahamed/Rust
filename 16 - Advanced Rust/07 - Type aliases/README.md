# Type aliases

A type alias gives an existing type another name. It can simplify signatures. A newtype creates a distinct type when you need stronger domain separation.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    type IoResult<T> = Result<T, std::io::Error>;
    let result: IoResult<u32> = Ok(9);
    assert_eq!(result.unwrap(), 9);
}
```

## 2. Common mistake

An alias does not stop callers from mixing two names for the same underlying type.

## Practice

Compare an alias for an identifier with a newtype that prevents mixing identifier kinds.

## References

- [Official Rust documentation · ch20 00 advanced features](https://doc.rust-lang.org/book/ch20-00-advanced-features.html)

---

[← Previous: Const generics](<../06 - Const generics/README.md>) | [Back to Advanced Rust](<../README.md>) | [Next: Type-state pattern →](<../08 - Type-state pattern/README.md>)
