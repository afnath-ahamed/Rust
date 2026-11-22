# Dynamic programming

Dynamic programming reuses overlapping subproblem results. Define the state, recurrence, base cases, and evaluation order before implementing memoization or iteration.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn ways(steps: u8) -> u64 {
    let (mut current, mut next) = (1, 1);
    for _ in 0..steps {
        (current, next) = (next, current + next);
    }
    current
}
assert_eq!(ways(0), 1);
assert_eq!(ways(5), 8);
```

## 2. Common mistake

Arithmetic overflow is independent of the algorithm's time complexity. Bound inputs or define an overflow policy.

## Practice

Compare naive recursion and the iterative approach, then add overflow handling.

## References

- [Official Rust documentation · collections](https://doc.rust-lang.org/std/collections/)

---

[← Previous: Recursion](<../09 - Recursion/README.md>) | [Back to Data Structures and Algorithms](<../README.md>) | [Next: Backend and Web Development →](<../../20 - Backend and Web Development/README.md>)
