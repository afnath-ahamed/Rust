# Borrowing rules

At a given time, ordinary access through references permits shared readers or exclusive mutable access. The compiler tracks where each reference is used, so a borrow often ends before the surrounding block ends.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let mut text = String::from("a");
    let view = &text;
    assert_eq!(view, "a");
    text.push('b');
    assert_eq!(text, "ab");
}
```

## 2. Common mistake

Using view again after push would extend the shared borrow across a mutation.

## Practice

Move the use of view after push, read the error, and restore a valid ordering.

## References

- [Official Rust documentation · ch04 02 references and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

---

[← Previous: Mutable borrowing](<../03 - Mutable borrowing/README.md>) | [Back to Borrowing and References](<../README.md>) | [Next: Borrow checker →](<../05 - Borrow checker/README.md>)
