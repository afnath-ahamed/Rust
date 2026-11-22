# Filter

filter keeps the items for which a predicate is true. The predicate receives a reference to the iterator item, so a borrowed iterator can introduce another layer of references.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let evens: Vec<_> = (1..=6).filter(|n| n % 2 == 0).collect();
    assert_eq!(evens, [2, 4, 6]);
}
```

## 2. Common mistake

Use filter_map when rejection and transformation belong in one operation.

## Practice

Keep parseable numbers from text using filter_map and compare it with filtering text first.

## References

- [Official Rust documentation · ch13 00 functional features](https://doc.rust-lang.org/book/ch13-00-functional-features.html)

---

[← Previous: Map](<../05 - Map/README.md>) | [Back to Functional Programming](<../README.md>) | [Next: Fold →](<../07 - Fold/README.md>)
