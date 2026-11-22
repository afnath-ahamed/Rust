# Trait implementation

Implement a trait with impl Trait for Type. Coherence prevents conflicting implementations, and the orphan rules generally require a local trait or a local type with additional constraints for foreign-trait implementations.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
struct Hours(u32);
impl std::fmt::Display for Hours {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} h", self.0)
    }
}
assert_eq!(Hours(3).to_string(), "3 h");
```

## 2. Common mistake

A newtype lets you provide behavior for your own wrapper when both original types are foreign.

## Practice

Add a displayable wrapper that renders a percentage.

## References

- [Official Rust documentation · ch10 00 generics](https://doc.rust-lang.org/book/ch10-00-generics.html)

---

[← Previous: Traits](<../04 - Traits/README.md>) | [Back to Generics, Traits and Lifetimes](<../README.md>) | [Next: Trait bounds →](<../06 - Trait bounds/README.md>)
