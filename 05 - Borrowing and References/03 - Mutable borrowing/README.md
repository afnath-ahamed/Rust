# Mutable borrowing

A mutable borrow allows exclusive access through &mut T. Methods can change the existing value and the original owner can use it again after the borrow ends.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    fn increment(total: &mut u32) {
        *total += 1;
    }
    let mut total = 2;
    increment(&mut total);
    assert_eq!(total, 3);
}
```

## 2. Common mistake

Declare the owner's binding mutable when creating a mutable borrow of its value.

## Practice

Use a mutable slice to set the first array element and handle an empty slice.

## Related lesson

[Variable scope](<../../04 - Ownership System/02 - Variable scope/README.md>)

## References

- [Official Rust documentation · ch04 02 references and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

---

[← Previous: Immutable borrowing](<../02 - Immutable borrowing/README.md>) | [Back to Borrowing and References](<../README.md>) | [Next: Borrowing rules →](<../04 - Borrowing rules/README.md>)
