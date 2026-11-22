# `Box<T>`

Box<T> gives one owner a heap-allocated value for nonzero-sized T. It is useful for recursive structures that need a finite-sized handle and for owning trait objects.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    enum List {
        End,
        Link(u32, Box<List>),
    }
    let list = List::Link(7, Box::new(List::End));
    if let List::Link(head, rest) = list {
        assert_eq!(head, 7);
        assert!(matches!(*rest, List::End));
    }
}
```

## 2. Common mistake

Box adds indirection and often allocation, so choose it for a concrete ownership or size benefit.

## Practice

Implement len for the recursive list and test the empty and two-node cases.

## References

- [Official Rust documentation · ch15 00 smart pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)

---

[← Previous: Functional Programming](<../../12 - Functional Programming/README.md>) | [Back to Smart Pointers](<../README.md>) | [Next: `Rc<T>` →](<../02 - Rc/README.md>)
