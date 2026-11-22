# Linked lists

A linked list stores each element in a separately linked node. Changing existing links can be cheap, but walking the list requires pointer chasing and usually has weaker locality than a Vec.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    use std::collections::LinkedList;
    let mut jobs = LinkedList::new();
    jobs.push_front("first");
    jobs.push_back("last");
    assert_eq!(jobs.pop_front(), Some("first"));
}
```

## 2. Common mistake

Accessing the nth node is linear and per-node allocation has overhead.

## Practice

Compare LinkedList and VecDeque for a queue of short strings.

## References

- [Official Rust documentation · collections](https://doc.rust-lang.org/std/collections/)

---

[← Previous: Sorting algorithms](<../03 - Sorting algorithms/README.md>) | [Back to Data Structures and Algorithms](<../README.md>) | [Next: Stacks →](<../05 - Stacks/README.md>)
