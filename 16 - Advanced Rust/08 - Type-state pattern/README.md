# Type-state pattern

Type-state encodes stages as distinct types. Transitions consume the old stage and return the new one, allowing the compiler to rule out calls out of order.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
struct Draft(String);
struct Published(String);
impl Draft {
    fn publish(self) -> Published {
        Published(self.0)
    }
}
impl Published {
    fn text(&self) -> &str {
        &self.0
    }
}
assert_eq!(Draft(String::from("entry")).publish().text(), "entry");
```

## 2. Common mistake

Use runtime state when states are selected dynamically and the extra type machinery would obscure simple flows.

## Practice

Add a validation transition and expose readout only for the valid state.

## References

- [Official Rust documentation · ch20 00 advanced features](https://doc.rust-lang.org/book/ch20-00-advanced-features.html)

---

[← Previous: Type aliases](<../07 - Type aliases/README.md>) | [Back to Advanced Rust](<../README.md>) | [Next: Higher-ranked trait bounds →](<../09 - Higher-ranked trait bounds/README.md>)
