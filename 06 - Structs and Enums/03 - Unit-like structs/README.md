# Unit-like structs

A unit-like struct has no fields and still has its own type. This is useful for stateless strategies, marker values, and trait implementations with no per-instance data.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
struct AlwaysReady;
impl AlwaysReady {
    fn ready(&self) -> bool {
        true
    }
}
assert!(AlwaysReady.ready());
```

## 2. Common mistake

A zero-sized marker can affect typing even though it stores no per-instance information.

## Practice

Implement a second stateless marker with a different method result.

## References

- [Official Rust documentation · ch05 00 structs](https://doc.rust-lang.org/book/ch05-00-structs.html)

---

[← Previous: Tuple structs](<../02 - Tuple structs/README.md>) | [Back to Structs and Enums](<../README.md>) | [Next: Methods →](<../04 - Methods/README.md>)
