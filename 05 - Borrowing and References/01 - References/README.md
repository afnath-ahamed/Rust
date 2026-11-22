# References

A reference gives temporary access to a value while another binding remains its owner. A shared reference has type &T, and a mutable reference has type &mut T.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    fn length(text: &str) -> usize {
        text.len()
    }
    let note = String::from("note");
    assert_eq!(length(&note), 4);
    assert_eq!(note, "note");
}
```

## 2. Common mistake

Calling a function with the owned value would transfer it when the type is non-Copy.

## Practice

Replace the string with a vector and write a function accepting a slice.

## Related lesson

[Ownership rules](<../../04 - Ownership System/01 - Ownership rules/README.md>)

## References

- [Official Rust documentation · ch04 02 references and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

---

[← Previous: Ownership System](<../../04 - Ownership System/README.md>) | [Back to Borrowing and References](<../README.md>) | [Next: Immutable borrowing →](<../02 - Immutable borrowing/README.md>)
