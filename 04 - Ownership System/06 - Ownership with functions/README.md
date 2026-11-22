# Ownership with functions

A by-value parameter receives ownership when its argument is moved. Copy arguments remain available to the caller because the callee receives a copy.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    fn consume(text: String) -> usize {
        text.len()
    }
    let message = String::from("hello");
    assert_eq!(consume(message), 5);
}
```

## 2. Common mistake

Avoid taking String when the only operation is inspection and callers need to retain the data.

## Practice

Write a version whose parameter is &str, then use the original String after the call.

## Related lesson

[Immutable borrowing](<../../05 - Borrowing and References/02 - Immutable borrowing/README.md>)

## References

- [Official Rust documentation · ch04 01 what is ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

---

[← Previous: Clone](<../05 - Clone/README.md>) | [Back to Ownership System](<../README.md>) | [Next: Return values and ownership →](<../07 - Return values and ownership/README.md>)
