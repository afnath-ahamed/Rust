# Unwrap and Expect

unwrap and expect both panic on Err or None. expect adds context about the broken assumption. They can be appropriate in tests and for invariants that are already established.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let expected_port = "443"
        .parse::<u16>()
        .expect("test fixture contains a valid port");
    assert_eq!(expected_port, 443);
}
```

## 2. Common mistake

Use explicit handling for arbitrary files, configuration, and user input.

## Practice

Rewrite a command-line argument unwrap as a validation error with a useful message.

## References

- [Official Rust documentation · ch09 00 error handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

---

[← Previous: Option](<../05 - Option/README.md>) | [Back to Error Handling](<../README.md>) | [Next: Error propagation (?) →](<../07 - Error propagation/README.md>)
