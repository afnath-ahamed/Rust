# Associated functions

An associated function belongs to an impl and has no self receiver. It is called through the type, often to construct a valid instance.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
struct Port(u16);
impl Port {
    fn new(value: u16) -> Option<Self> {
        (value != 0).then_some(Self(value))
    }
}
assert_eq!(Port::new(443).map(|port| port.0), Some(443));
```

## 2. Common mistake

new is a convention. Rust has no special constructor keyword or automatic validation.

## Practice

Add a constructor that parses a string and keeps parse errors distinct from a forbidden zero.

## References

- [Official Rust documentation · ch05 00 structs](https://doc.rust-lang.org/book/ch05-00-structs.html)

---

[← Previous: Methods](<../04 - Methods/README.md>) | [Back to Structs and Enums](<../README.md>) | [Next: Implementation blocks (impl) →](<../06 - Implementation blocks (impl)/README.md>)
