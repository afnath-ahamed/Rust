# Tuples

A tuple groups a fixed number of potentially different types. Positional access and destructuring are convenient for small return values. Named structs tend to read better when fields carry business meanings.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    fn origin() -> (i32, i32) {
        (0, 0)
    }
    let (x, y) = origin();
    assert_eq!(x + y, 0);
}
```

## 2. Common mistake

A one-element tuple requires a trailing comma: (value,).

## Practice

Return a minimum and maximum in a tuple, then name the fields in a struct version.

## References

- [Official Rust documentation · collections](https://doc.rust-lang.org/std/collections/)

---

[← Previous: Arrays](<../01 - Arrays/README.md>) | [Back to Collections and Strings](<../README.md>) | [Next: Vectors →](<../03 - Vectors/README.md>)
