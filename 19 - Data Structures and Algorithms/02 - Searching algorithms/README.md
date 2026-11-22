# Searching algorithms

Linear search handles unsorted input in linear time. Binary search uses a sorted sequence to discard half of the remaining range at each comparison.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let ordered = [2, 5, 8, 11];
    assert_eq!(ordered.binary_search(&8), Ok(2));
    assert_eq!(ordered.binary_search(&7), Err(2));
}
```

## 2. Common mistake

Binary search requires the same ordering used to build the sequence. A missing key yields an insertion position.

## Practice

Track a missing search target by hand and verify the insertion point.

## References

- [Official Rust documentation · collections](https://doc.rust-lang.org/std/collections/)

---

[← Previous: Time and space complexity](<../01 - Time and space complexity/README.md>) | [Back to Data Structures and Algorithms](<../README.md>) | [Next: Sorting algorithms →](<../03 - Sorting algorithms/README.md>)
