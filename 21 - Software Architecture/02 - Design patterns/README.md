# Design patterns

Design patterns name recurring structures such as Adapter, Strategy, and Builder. Choose a pattern once it clarifies a real relationship. Rust often expresses a strategy with a closure or trait and a fixed state set with an enum.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn price(base: u64, strategy: impl Fn(u64) -> u64) -> u64 {
    strategy(base)
}
assert_eq!(price(100, |amount| amount - 10), 90);
```

## 2. Common mistake

Callbacks should have clear ownership and error rules even when their implementation is small.

## Practice

Replace the closure with two named strategies and document their behavior.

## References

- [Official Rust documentation · ch07 00 managing growing projects with packages crates and modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

[← Previous: SOLID principles](<../01 - SOLID principles/README.md>) | [Back to Software Architecture](<../README.md>) | [Next: Dependency injection →](<../03 - Dependency injection/README.md>)
