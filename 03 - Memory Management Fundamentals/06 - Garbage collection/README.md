# Garbage collection

Tracing garbage collectors typically reclaim objects found unreachable from program roots. Rust normally ties cleanup to ownership and destruction. Reference-counted owners free their value when the last strong owner goes away.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    use std::rc::Rc;
    let first = Rc::new(String::from("note"));
    let second = Rc::clone(&first);
    drop(first);
    assert_eq!(Rc::strong_count(&second), 1);
}
```

## 2. Common mistake

Reference counting alone cannot reclaim a cycle of strong references.

## Practice

Draw two objects that own each other and explain why the count never reaches zero.

## References

- [Official Rust documentation · ch04 01 what is ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

---

[← Previous: Pointers](<../05 - Pointers/README.md>) | [Back to Memory Management Fundamentals](<../README.md>) | [Next: Dangling pointers →](<../07 - Dangling pointers/README.md>)
