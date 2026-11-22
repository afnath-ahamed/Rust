# Error propagation (?)

The ? operator extracts the success value or returns early with a compatible error. It also propagates None in Option-returning code. Conversion between Result and Option is explicit.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn sum_inputs(a: &str, b: &str) -> Result<u32, std::num::ParseIntError> {
    Ok(u32::from(a.parse::<u16>()?) + u32::from(b.parse::<u16>()?))
}
assert_eq!(sum_inputs("2", "3").unwrap(), 5);
assert!(sum_inputs("x", "3").is_err());
```

## 2. Common mistake

Early propagation can lose the context of which operation failed unless context is attached at a boundary.

## Practice

Add error context for the first and second input separately.

## References

- [Official Rust documentation · ch09 00 error handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

---

[← Previous: Unwrap and Expect](<../06 - Unwrap and Expect/README.md>) | [Back to Error Handling](<../README.md>) | [Next: Custom errors →](<../08 - Custom errors/README.md>)
