# Arrays

An array has a fixed length encoded in its type. It is useful for a known number of same-typed values. Pass a slice when a function can accept more than one length.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let samples: [i32; 3] = [3, 5, 7];
    assert_eq!(samples.iter().sum::<i32>(), 15);
    assert_eq!(samples.get(3), None);
}
```

## 2. Common mistake

[T; 3] and [T; 4] are distinct types.

## Practice

Write a function that totals both arrays through a slice parameter.

## References

- [Official Rust documentation · collections](https://doc.rust-lang.org/std/collections/)

---

[← Previous: Pattern Matching](<../../07 - Pattern Matching/README.md>) | [Back to Collections and Strings](<../README.md>) | [Next: Tuples →](<../02 - Tuples/README.md>)
