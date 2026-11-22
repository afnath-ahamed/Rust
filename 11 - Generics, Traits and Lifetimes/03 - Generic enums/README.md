# Generic enums

Generic enums let variant payloads vary by use while fixing the available states. Option and Result are standard examples; domain enums can use the same approach.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    enum Load<T> {
        Empty,
        Ready(T),
    }
    let values = [Load::Empty, Load::Ready(21)];
    assert!(matches!(values[1], Load::Ready(21)));
}
```

## 2. Common mistake

An empty variant may leave the payload type ambiguous without surrounding context.

## Practice

Add a failure variant with a separate error type parameter.

## References

- [Official Rust documentation · ch10 00 generics](https://doc.rust-lang.org/book/ch10-00-generics.html)

---

[← Previous: Generic structs](<../02 - Generic structs/README.md>) | [Back to Generics, Traits and Lifetimes](<../README.md>) | [Next: Traits →](<../04 - Traits/README.md>)
