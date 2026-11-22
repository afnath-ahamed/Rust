# Lifetime annotations

Lifetime annotations describe how references relate. They do not extend the lifetime of an input. Tie the output to the actual source borrow when some inputs are independent.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn prefix<'a>(text: &'a str, fallback: &str) -> &'a str {
    let _ = fallback;
    text.split_whitespace().next().unwrap_or("")
}
assert_eq!(prefix("hello rust", "empty"), "hello");
```

## 2. Common mistake

Using one lifetime for all parameters can unnecessarily narrow an output borrow.

## Practice

Write a chooser that can return either input and explain why its output has a shared lifetime bound.

## References

- [Official Rust documentation · ch10 00 generics](https://doc.rust-lang.org/book/ch10-00-generics.html)

---

[← Previous: Associated types](<../08 - Associated types/README.md>) | [Back to Generics, Traits and Lifetimes](<../README.md>) | [Next: Lifetime elision →](<../10 - Lifetime elision/README.md>)
