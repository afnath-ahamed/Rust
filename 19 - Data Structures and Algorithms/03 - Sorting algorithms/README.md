# Sorting algorithms

Compare sort algorithms by stability, auxiliary storage, comparison cost, and expected inputs. Stable sorting preserves the relative order of equal keys. Unstable sorting can be appropriate when ties carry no meaning.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let mut rows = [(2, "first"), (1, "other"), (2, "second")];
    rows.sort_by_key(|row| row.0);
    assert_eq!(rows[1..], [(2, "first"), (2, "second")]);
}
```

## 2. Common mistake

Comparators must define a consistent order. Floating-point sorting needs a deliberate NaN policy such as total_cmp.

## Practice

Implement insertion sort for small slices and compare its output with slice::sort.

## References

- [Official Rust documentation · collections](https://doc.rust-lang.org/std/collections/)

---

[← Previous: Searching algorithms](<../02 - Searching algorithms/README.md>) | [Back to Data Structures and Algorithms](<../README.md>) | [Next: Linked lists →](<../04 - Linked lists/README.md>)
