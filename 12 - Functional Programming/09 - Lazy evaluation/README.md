# Lazy evaluation

Most adapters defer computation until a consumer requests items. Consumers such as find and any may stop early, allowing pipelines to work with otherwise unbounded iterators.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let first_three: Vec<_> = (0..).map(|n| n * 2).take(3).collect();
    assert_eq!(first_three, [0, 2, 4]);
}
```

## 2. Common mistake

An infinite iterator needs a terminating condition before a consumer like collect.

## Practice

Use inspect with a Cell counter and verify how many items find visits.

## References

- [Official Rust documentation · ch13 00 functional features](https://doc.rust-lang.org/book/ch13-00-functional-features.html)

---

[← Previous: Collect](<../08 - Collect/README.md>) | [Back to Functional Programming](<../README.md>) | [Next: Zero-cost abstractions →](<../10 - Zero-cost abstractions/README.md>)
