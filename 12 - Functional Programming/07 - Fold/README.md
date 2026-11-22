# Fold

fold combines items into an accumulator starting at an explicit initial value. Its output type can differ from the item type. The initial value determines the outcome for empty input.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let (sum, count) = [3u64, 5, 7]
        .into_iter()
        .fold((0, 0), |(sum, count), n| (sum + n, count + 1));
    assert_eq!((sum, count), (15, 3));
}
```

## 2. Common mistake

Use sum or count when either alone already expresses the operation.

## Practice

Use try_fold with checked_add and show the result for an overflowing u8 sum.

## References

- [Official Rust documentation · ch13 00 functional features](https://doc.rust-lang.org/book/ch13-00-functional-features.html)

---

[← Previous: Filter](<../06 - Filter/README.md>) | [Back to Functional Programming](<../README.md>) | [Next: Collect →](<../08 - Collect/README.md>)
