# Unrecoverable errors

An unrecoverable condition is one where the component cannot continue with its invariants intact. The deployment or calling boundary still decides how to isolate and restart failed work.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn invariant_holds(items: &[u32]) -> bool {
    items.windows(2).all(|p| p[0] <= p[1])
}
assert!(invariant_holds(&[1, 1, 2]));
```

## 2. Common mistake

Invalid data from a client is ordinarily an expected failure, not an internal invariant violation.

## Practice

List three failure cases and assign each one a validation, recoverable error, or invariant response.

## References

- [Official Rust documentation · ch09 00 error handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

---

[← Previous: Recoverable errors](<../01 - Recoverable errors/README.md>) | [Back to Error Handling](<../README.md>) | [Next: Panic →](<../03 - Panic/README.md>)
