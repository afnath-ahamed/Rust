# Recursion

Recursion reduces a problem to smaller instances and stops at a base case. Choose an iterative implementation when depth can grow uncontrolled.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn sum(values: &[i32]) -> i32 {
    match values.split_first() {
        None => 0,
        Some((head, tail)) => *head + sum(tail),
    }
}
assert_eq!(sum(&[1, 3, 5]), 9);
```

## 2. Common mistake

Rust does not promise tail-call optimization.

## Practice

Rewrite the recursive sum using an explicit loop and compare its stack usage.

## References

- [Official Rust documentation · collections](https://doc.rust-lang.org/std/collections/)

---

[← Previous: Graphs](<../08 - Graphs/README.md>) | [Back to Data Structures and Algorithms](<../README.md>) | [Next: Dynamic programming →](<../10 - Dynamic programming/README.md>)
