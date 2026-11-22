# Dangling references

A reference must remain valid for every use. Returning owned data is appropriate when a function creates the data. Returning a reference can reuse data that the caller already owns.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn prefix(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}
assert_eq!(prefix("safe Rust"), "safe");
```

## 2. Common mistake

A lifetime annotation records relationships between borrows and cannot keep a destroyed local value alive.

## Practice

Try returning a slice of a locally allocated String and identify which owner would be destroyed.

## References

- [Official Rust documentation · ch04 02 references and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

---

[← Previous: Borrow checker](<../05 - Borrow checker/README.md>) | [Back to Borrowing and References](<../README.md>) | [Next: Dereferencing →](<../07 - Dereferencing/README.md>)
