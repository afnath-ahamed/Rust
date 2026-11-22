# Map

map transforms each item into a value of another type or representation. Use a closure for domain logic or a function for a named reusable transformation.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let lengths: Vec<_> = ["rust", "go"].iter().map(|word| word.len()).collect();
    assert_eq!(lengths, [4, 2]);
}
```

## 2. Common mistake

map can be skipped entirely when its adapter is never consumed.

## Practice

Map a list of temperatures from Celsius to Fahrenheit and verify two known values.

## References

- [Official Rust documentation · ch13 00 functional features](https://doc.rust-lang.org/book/ch13-00-functional-features.html)

---

[← Previous: Iterator adapters](<../04 - Iterator adapters/README.md>) | [Back to Functional Programming](<../README.md>) | [Next: Filter →](<../06 - Filter/README.md>)
