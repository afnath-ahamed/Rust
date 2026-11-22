# HashSet

HashSet tracks unique keys and provides set operations such as intersection. Use it for membership tests or deduplication when order is irrelevant.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    use std::collections::HashSet;
    let first = HashSet::from([1, 2, 3]);
    let second = HashSet::from([3, 4]);
    assert_eq!(
        first.intersection(&second).copied().collect::<Vec<_>>(),
        [3]
    );
}
```

## 2. Common mistake

Changing a key's equality or hash while stored breaks the collection's assumptions.

## Practice

Find items unique to each of two normalized word lists.

## References

- [Official Rust documentation · collections](https://doc.rust-lang.org/std/collections/)

---

[← Previous: HashMap](<../05 - HashMap/README.md>) | [Back to Collections and Strings](<../README.md>) | [Next: BTreeMap →](<../07 - BTreeMap/README.md>)
