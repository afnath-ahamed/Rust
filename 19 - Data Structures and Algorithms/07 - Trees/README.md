# Trees

Trees contain parent-child relationships and are useful for hierarchy and ordered lookup. An unbalanced binary search tree can degenerate toward a list, so standard ordered collections use strategies to control shape.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    use std::collections::BTreeSet;
    let nodes = BTreeSet::from([7, 2, 9, 4]);
    assert_eq!(nodes.range(3..8).copied().collect::<Vec<_>>(), [4, 7]);
}
```

## 2. Common mistake

Searching a hierarchy must account for whether the tree is actually ordered.

## Practice

Write a depth-first traversal of a folder tree using a stack.

## References

- [Official Rust documentation · collections](https://doc.rust-lang.org/std/collections/)

---

[← Previous: Queues](<../06 - Queues/README.md>) | [Back to Data Structures and Algorithms](<../README.md>) | [Next: Graphs →](<../08 - Graphs/README.md>)
