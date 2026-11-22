# `Cell<T>`

Cell<T> supports mutation through a shared reference by moving or copying values instead of lending references to its contents. get works when T is Copy; replace works more broadly.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    use std::cell::Cell;
    let count = Cell::new(2);
    let old = count.replace(9);
    assert_eq!((old, count.get()), (2, 9));
}
```

## 2. Common mistake

Cell is useful for local counters and is not a cross-thread synchronization tool.

## Practice

Implement an access counter updated by a method taking &self.

## References

- [Official Rust documentation · ch15 00 smart pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)

---

[← Previous: `RefCell<T>`](<../04 - RefCell/README.md>) | [Back to Smart Pointers](<../README.md>) | [Next: `Weak<T>` →](<../06 - Weak/README.md>)
