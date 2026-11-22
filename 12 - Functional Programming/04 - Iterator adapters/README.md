# Iterator adapters

Iterator adapters transform pipelines lazily. chain appends a second sequence, enumerate adds an index, and take limits the number of items passed downstream.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let rows: Vec<_> = ["a", "b"]
        .into_iter()
        .chain(["c"])
        .enumerate()
        .take(2)
        .collect();
    assert_eq!(rows, [(0, "a"), (1, "b")]);
}
```

## 2. Common mistake

Adapter order affects both semantics and the work performed.

## Practice

Compare take(2).filter(...) and filter(...).take(2) with an input containing rejected items.

## References

- [Official Rust documentation · ch13 00 functional features](https://doc.rust-lang.org/book/ch13-00-functional-features.html)

---

[← Previous: Iterators](<../03 - Iterators/README.md>) | [Back to Functional Programming](<../README.md>) | [Next: Map →](<../05 - Map/README.md>)
