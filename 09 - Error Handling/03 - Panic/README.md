# Panic

panic! stops normal execution on the current thread. A panic may unwind or abort depending on configuration. Use it for invariant failures rather than ordinary decision paths.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn checked_index(values: &[i32], index: usize) -> i32 {
    assert!(index < values.len(), "internal index must be valid");
    values[index]
}
assert_eq!(checked_index(&[7], 0), 7);
```

## 2. Common mistake

catch_unwind does not catch panics that abort and is not routine error handling.

## Practice

Write a test for a panicking invariant using should_panic with an expected message.

## References

- [Official Rust documentation · ch09 00 error handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

---

[← Previous: Unrecoverable errors](<../02 - Unrecoverable errors/README.md>) | [Back to Error Handling](<../README.md>) | [Next: Result →](<../04 - Result/README.md>)
