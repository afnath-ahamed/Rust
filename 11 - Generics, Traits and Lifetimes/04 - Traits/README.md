# Traits

A trait describes behavior shared by types. Its methods and associated items form a contract that users can reference in generic signatures.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
trait Summarize {
    fn label(&self) -> &'static str;
}
struct Job;
impl Summarize for Job {
    fn label(&self) -> &'static str {
        "job"
    }
}
assert_eq!(Job.label(), "job");
```

## 2. Common mistake

Make trait contracts say which inputs and effects callers can rely on.

## Practice

Create two types implementing the same measurement trait and call both.

## References

- [Official Rust documentation · ch10 00 generics](https://doc.rust-lang.org/book/ch10-00-generics.html)

---

[← Previous: Generic enums](<../03 - Generic enums/README.md>) | [Back to Generics, Traits and Lifetimes](<../README.md>) | [Next: Trait implementation →](<../05 - Trait implementation/README.md>)
