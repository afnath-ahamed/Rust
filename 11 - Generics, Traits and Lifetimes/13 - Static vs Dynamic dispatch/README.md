# Static vs Dynamic dispatch

Generic calls commonly use static dispatch with concrete instantiations. Calling through dyn Trait typically uses dynamic dispatch. Optimization and code-size effects depend on the compiled program.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn fixed(value: &impl std::fmt::Display) -> String {
    value.to_string()
}
fn dynamic(value: &dyn std::fmt::Display) -> String {
    value.to_string()
}
assert_eq!(fixed(&7), dynamic(&7));
```

## 2. Common mistake

A trait object can be borrowed. Dynamic dispatch does not itself require a heap allocation.

## Practice

Choose the version you would use for a collection of heterogeneous plugin instances and explain why.

## References

- [Official Rust documentation · ch10 00 generics](https://doc.rust-lang.org/book/ch10-00-generics.html)

---

[← Previous: Trait objects](<../12 - Trait objects/README.md>) | [Back to Generics, Traits and Lifetimes](<../README.md>) | [Next: Functional Programming →](<../../12 - Functional Programming/README.md>)
