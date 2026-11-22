# Reference cycles

Strong reference cycles leave counts above zero after external owners disappear. Model ownership edges deliberately and use Weak for back-references or observers.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    use std::rc::{Rc, Weak};
    struct Node {
        parent: Weak<Node>,
    }
    let parent = Rc::new(Node {
        parent: Weak::new(),
    });
    let child = Rc::new(Node {
        parent: Rc::downgrade(&parent),
    });
    drop(parent);
    assert!(child.parent.upgrade().is_none());
}
```

## 2. Common mistake

A graph with strong ownership in both directions can leak while remaining memory safe.

## Practice

Draw a parent-child graph with labeled strong and weak edges and trace the destruction order.

## References

- [Official Rust documentation · ch15 00 smart pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)

---

[← Previous: Interior mutability](<../10 - Interior mutability/README.md>) | [Back to Smart Pointers](<../README.md>) | [Next: Concurrency and Parallelism →](<../../14 - Concurrency and Parallelism/README.md>)
