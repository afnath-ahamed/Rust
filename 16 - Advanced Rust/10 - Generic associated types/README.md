# Generic associated types

Generic associated types let an associated type depend on generic arguments such as the lifetime of a borrow. This can model an item borrowed from the receiver's current access.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
trait View {
    type Item<'a>
    where
        Self: 'a;
    fn view(&self) -> Self::Item<'_>;
}
impl View for String {
    type Item<'a> = &'a str;
    fn view(&self) -> Self::Item<'_> {
        self.as_str()
    }
}
assert_eq!(String::from("rust").view(), "rust");
```

## 2. Common mistake

Identify which borrow constrains each associated lifetime before adding a generic associated type.

## Practice

Implement View for a Vec<u8> with a borrowed slice as its item.

## References

- [Official Rust documentation · ch20 00 advanced features](https://doc.rust-lang.org/book/ch20-00-advanced-features.html)

---

[← Previous: Higher-ranked trait bounds](<../09 - Higher-ranked trait bounds/README.md>) | [Back to Advanced Rust](<../README.md>) | [Next: Declarative macros →](<../11 - Declarative macros/README.md>)
