# Vectors

Vec<T> stores a growable sequence, with a length for live elements and a capacity for reserved storage. Use get for untrusted indexes and slices for read-only views over many positions.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let mut items = Vec::with_capacity(3);
    items.extend([4, 6, 8]);
    assert_eq!(items.pop(), Some(8));
    assert_eq!(items.as_slice(), &[4, 6]);
}
```

## 2. Common mistake

Reallocation can move elements, so the borrow checker prevents mutating a vector while a used element borrow remains active.

## Practice

Collect odd numbers in a vector and compare growth with and without reserving capacity.

## References

- [Official Rust documentation · collections](https://doc.rust-lang.org/std/collections/)

---

[← Previous: Tuples](<../02 - Tuples/README.md>) | [Back to Collections and Strings](<../README.md>) | [Next: String vs &str →](<../04 - String vs Str/README.md>)
