# `Rc<T>`

Rc<T> provides shared, single-threaded ownership with strong reference counts. Cloning an Rc increments the count. The contained value is destroyed when the last strong owner disappears.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    use std::rc::Rc;
    let label = Rc::new(String::from("shared"));
    let alias = Rc::clone(&label);
    assert!(Rc::ptr_eq(&label, &alias));
    assert_eq!(Rc::strong_count(&label), 2);
}
```

## 2. Common mistake

Rc cannot be sent between threads, and strong cycles can keep values alive.

## Practice

Add a third owner, drop two owners, and verify the remaining count.

## References

- [Official Rust documentation · ch15 00 smart pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)

---

[← Previous: `Box<T>`](<../01 - Box/README.md>) | [Back to Smart Pointers](<../README.md>) | [Next: `Arc<T>` →](<../03 - Arc/README.md>)
