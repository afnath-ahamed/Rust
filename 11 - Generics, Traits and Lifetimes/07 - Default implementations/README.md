# Default implementations

A default implementation supplies behavior derived from a trait's contract. Implementers can override the default and need only supply required methods.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
trait SizedReport {
    fn count(&self) -> usize;
    fn empty(&self) -> bool {
        self.count() == 0
    }
}
impl SizedReport for Vec<u8> {
    fn count(&self) -> usize {
        self.len()
    }
}
assert!(Vec::<u8>::new().empty());
```

## 2. Common mistake

A default should preserve the documented behavior for every valid implementation.

## Practice

Override the default for a fixed-capacity type without changing its observable meaning.

## References

- [Official Rust documentation · ch10 00 generics](https://doc.rust-lang.org/book/ch10-00-generics.html)

---

[← Previous: Trait bounds](<../06 - Trait bounds/README.md>) | [Back to Generics, Traits and Lifetimes](<../README.md>) | [Next: Associated types →](<../08 - Associated types/README.md>)
