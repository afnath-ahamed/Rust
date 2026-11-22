# Ownership rules

Every value has an owner. Transferring an owned value changes which binding can use it. When an owning binding goes out of scope without moving the value onward, the value is dropped.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let draft = String::from("draft");
    let published = draft;
    assert_eq!(published, "draft");
}
```

## 2. Common mistake

A binding is a name. Two bindings do not automatically share ownership of a String.

## Practice

Explain why the original name cannot be used and how an explicit clone would change the situation.

## References

- [Official Rust documentation · ch04 01 what is ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

---

[← Previous: Memory Management Fundamentals](<../../03 - Memory Management Fundamentals/README.md>) | [Back to Ownership System](<../README.md>) | [Next: Variable scope →](<../02 - Variable scope/README.md>)
