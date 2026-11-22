# Let else

Use let else to bind a success pattern and exit the current path if it fails. The else block must diverge, and the new bindings remain available in the enclosing scope after the statement.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn first(values: &[i32]) -> i32 {
    let [head, ..] = values else {
        return 0;
    };
    *head
}
assert_eq!((first(&[8, 9]), first(&[])), (8, 0));
```

## 2. Common mistake

An else block that simply calculates a fallback value does not satisfy the required divergence.

## Practice

In a loop, use let else and continue to skip empty records.

## References

- [Official Rust documentation · ch19 00 patterns](https://doc.rust-lang.org/book/ch19-00-patterns.html)

---

[← Previous: While let](<../03 - While let/README.md>) | [Back to Pattern Matching](<../README.md>) | [Next: Destructuring →](<../05 - Destructuring/README.md>)
