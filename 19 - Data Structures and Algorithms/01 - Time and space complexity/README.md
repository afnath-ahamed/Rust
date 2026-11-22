# Time and space complexity

Time complexity describes how work grows with input size. Space complexity describes extra storage growth under the stated model. State the input size, operation counted, and usual or worst case assumptions.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn touches(n: usize) -> usize {
    (0..n).flat_map(|_| 0..n).count()
}
assert_eq!(touches(3), 9);
assert_eq!(touches(6), 36);
```

## 2. Common mistake

Big-O omits constant factors and does not predict timings for small inputs.

## Practice

Compare the operation counts of one pass, two independent passes, and a nested pass.

## References

- [Official Rust documentation · collections](https://doc.rust-lang.org/std/collections/)

---

[← Previous: Professional Rust Tooling](<../../18 - Professional Rust Tooling/README.md>) | [Back to Data Structures and Algorithms](<../README.md>) | [Next: Searching algorithms →](<../02 - Searching algorithms/README.md>)
