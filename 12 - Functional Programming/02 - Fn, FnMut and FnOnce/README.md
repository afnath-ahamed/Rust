# `Fn / FnMut / FnOnce`

Fn supports repeated shared calls, FnMut may change captured state, and FnOnce permits consuming captured values. Choose the weakest call-trait requirement compatible with the API's behavior.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    fn twice(mut work: impl FnMut()) {
        work();
        work();
    }
    let mut calls = 0;
    twice(|| calls += 1);
    assert_eq!(calls, 2);
}
```

## 2. Common mistake

A closure moving a captured value out of itself on a call is callable only once.

## Practice

Write a function taking FnOnce and pass a closure that returns an owned capture.

## References

- [Official Rust documentation · ch13 00 functional features](https://doc.rust-lang.org/book/ch13-00-functional-features.html)

---

[← Previous: Closures](<../01 - Closures/README.md>) | [Back to Functional Programming](<../README.md>) | [Next: Iterators →](<../03 - Iterators/README.md>)
