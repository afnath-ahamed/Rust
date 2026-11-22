# Dereferencing

Dereferencing with * accesses the value through a reference. Method calls and field access frequently apply dereferencing automatically, which is why many method calls omit an explicit *.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let mut value = 10;
    let reference = &mut value;
    *reference += 2;
    assert_eq!(value, 12);
}
```

## 2. Common mistake

Reading *reference by value cannot move a non-Copy value out through an ordinary reference.

## Practice

Compare assigning to *reference with reassigning a mutable binding that stores a reference.

## References

- [Official Rust documentation · ch04 02 references and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

---

[← Previous: Dangling references](<../06 - Dangling references/README.md>) | [Back to Borrowing and References](<../README.md>) | [Next: Slices →](<../08 - Slices/README.md>)
