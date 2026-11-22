# Lifetime elision

Lifetime elision rules infer common relationships in signatures. A single input lifetime is assigned to elided output lifetimes, and a method receiver often determines the output borrow.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}
assert_eq!(first_word("Rust notes"), "Rust");
```

## 2. Common mistake

Two independent input borrows with no receiver commonly require an explicit output relationship.

## Practice

Rewrite first_word with explicit names for its input and output lifetime.

## References

- [Official Rust documentation · ch10 00 generics](https://doc.rust-lang.org/book/ch10-00-generics.html)

---

[← Previous: Lifetime annotations](<../09 - Lifetime annotations/README.md>) | [Back to Generics, Traits and Lifetimes](<../README.md>) | [Next: Static lifetime →](<../11 - Static lifetime/README.md>)
