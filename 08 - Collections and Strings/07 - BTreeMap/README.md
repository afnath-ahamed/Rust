# BTreeMap

BTreeMap maintains keys in sorted order and supports range searches. Choose it when predictable ordering or ordered queries are central to the task.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    use std::collections::BTreeMap;
    let readings = BTreeMap::from([(1, 20), (3, 24), (5, 30)]);
    let selected: Vec<_> = readings
        .range(2..=4)
        .map(|(&day, &value)| (day, value))
        .collect();
    assert_eq!(selected, [(3, 24)]);
}
```

## 2. Common mistake

Its ordering comes from the key's Ord implementation.

## Practice

Use a BTreeMap to return readings for an inclusive date range.

## References

- [Official Rust documentation · collections](https://doc.rust-lang.org/std/collections/)

---

[← Previous: HashSet](<../06 - HashSet/README.md>) | [Back to Collections and Strings](<../README.md>) | [Next: VecDeque →](<../08 - VecDeque/README.md>)
