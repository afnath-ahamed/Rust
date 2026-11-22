# Generic structs

A generic struct stores data while keeping its type relationships visible. An impl can cover every T or specialize an inherent implementation for a concrete type.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    struct Point<T> {
        x: T,
        y: T,
    }
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    let p = Point { x: 3, y: 8 };
    assert_eq!((*p.x(), p.y), (3, 8));
}
```

## 2. Common mistake

A method using ordering needs a bound or a different implementation.

## Practice

Write a Pair<T, U> type and add an accessor for each field.

## References

- [Official Rust documentation · ch10 00 generics](https://doc.rust-lang.org/book/ch10-00-generics.html)

---

[← Previous: Generic functions](<../01 - Generic functions/README.md>) | [Back to Generics, Traits and Lifetimes](<../README.md>) | [Next: Generic enums →](<../03 - Generic enums/README.md>)
