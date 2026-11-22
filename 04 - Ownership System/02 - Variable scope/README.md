# Variable scope

A local binding is visible from its declaration through the enclosing block, subject to shadowing. An inner block can use outer bindings. A value can leave a block by being moved into its result.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let title = {
        let local = String::from("Rust");
        local
    };
    assert_eq!(title, "Rust");
}
```

## 2. Common mistake

The name local ends at the brace even though its value survives under a different name.

## Practice

Add an inner block which shadows title and show what each scope prints.

## References

- [Official Rust documentation · ch04 01 what is ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

---

[← Previous: Ownership rules](<../01 - Ownership rules/README.md>) | [Back to Ownership System](<../README.md>) | [Next: Move semantics →](<../03 - Move semantics/README.md>)
