# `Weak<T>`

Weak<T> references an Rc or Arc allocation without keeping its value alive. upgrade returns Option because the strong owners may have gone away.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    use std::rc::Rc;
    let owner = Rc::new(5);
    let weak = Rc::downgrade(&owner);
    assert!(weak.upgrade().is_some());
    drop(owner);
    assert!(weak.upgrade().is_none());
}
```

## 2. Common mistake

Avoid assuming upgrade always succeeds.

## Practice

Use a weak link for a child's parent and handle the parent being gone.

## References

- [Official Rust documentation · ch15 00 smart pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)

---

[← Previous: `Cell<T>`](<../05 - Cell/README.md>) | [Back to Smart Pointers](<../README.md>) | [Next: `Cow<T>` →](<../07 - Cow/README.md>)
