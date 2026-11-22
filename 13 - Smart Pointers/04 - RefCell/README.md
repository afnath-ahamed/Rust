# `RefCell<T>`

RefCell<T> enforces borrowing rules at runtime. borrow and borrow_mut panic on violations. try_borrow and try_borrow_mut report an error instead.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    use std::cell::RefCell;
    let count = RefCell::new(1);
    let reader = count.borrow();
    assert!(count.try_borrow_mut().is_err());
    drop(reader);
    *count.borrow_mut() = 2;
    assert_eq!(*count.borrow(), 2);
}
```

## 2. Common mistake

Keep guard scopes short so nested calls do not accidentally overlap borrows.

## Practice

Refactor a function to release a borrow guard before invoking a callback.

## References

- [Official Rust documentation · ch15 00 smart pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)

---

[← Previous: `Arc<T>`](<../03 - Arc/README.md>) | [Back to Smart Pointers](<../README.md>) | [Next: `Cell<T>` →](<../05 - Cell/README.md>)
