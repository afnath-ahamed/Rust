# Higher-ranked trait bounds

Higher-ranked bounds can require behavior for every lifetime. The spelling for<'a> commonly appears when a callback must accept temporary borrows supplied independently by its caller.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn apply<F>(f: F) -> usize
where
    F: for<'a> Fn(&'a str) -> usize,
{
    let local = String::from("Rust");
    f(&local)
}
assert_eq!(apply(str::len), 4);
```

## 2. Common mistake

A bound for one outside lifetime would not promise acceptance of this function's temporary borrow.

## Practice

Pass a function that reads a borrowed slice and use it with two different local owners.

## References

- [Official Rust documentation · ch20 00 advanced features](https://doc.rust-lang.org/book/ch20-00-advanced-features.html)

---

[← Previous: Type-state pattern](<../08 - Type-state pattern/README.md>) | [Back to Advanced Rust](<../README.md>) | [Next: Generic associated types →](<../10 - Generic associated types/README.md>)
