# Generic functions

A generic function represents a family of functions parameterized by types. Its body may use only operations the signature guarantees for those types.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn pair<T>(a: T, b: T) -> (T, T) {
    (a, b)
}
assert_eq!(pair(3, 4), (3, 4));
assert_eq!(pair("a", "b"), ("a", "b"));
```

## 2. Common mistake

Using the same T for two arguments ties their types together.

## Practice

Change pair to support unrelated types and call it with a number and a string.

## References

- [Official Rust documentation · ch10 00 generics](https://doc.rust-lang.org/book/ch10-00-generics.html)

---

[← Previous: Project Organization](<../../10 - Project Organization/README.md>) | [Back to Generics, Traits and Lifetimes](<../README.md>) | [Next: Generic structs →](<../02 - Generic structs/README.md>)
