# Borrow checker

The borrow checker rejects uses that violate ownership, aliasing, or lifetime requirements. Begin with the diagnostic span where a borrow starts and trace the conflicting mutation or last reference use.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let mut items = vec![1, 2];
    let first_value = items[0];
    items.push(3);
    assert_eq!(first_value, 1);
}
```

## 2. Common mistake

This example copies an i32. Borrowing the element instead and using it after push could reference moved storage.

## Practice

Get two mutable slices from one array with split_at_mut and explain why they can coexist.

## References

- [Official Rust documentation · ch04 02 references and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

---

[← Previous: Borrowing rules](<../04 - Borrowing rules/README.md>) | [Back to Borrowing and References](<../README.md>) | [Next: Dangling references →](<../06 - Dangling references/README.md>)
