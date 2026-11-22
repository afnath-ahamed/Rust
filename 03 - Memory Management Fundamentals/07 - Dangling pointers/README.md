# Dangling pointers

A dangling pointer outlives the storage it points to. Rust's safe reference rules prevent returning a borrow of a local value that will be destroyed at function exit.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn label() -> String {
    String::from("owned text")
}
assert_eq!(label(), "owned text");
```

## 2. Common mistake

A raw pointer can remain representable after storage goes away. Reading through it is invalid.

## Practice

Try changing the function to return a reference to a local String and read the compiler diagnostic.

## Related lesson

[Drop](<../../04 - Ownership System/08 - Drop/README.md>)

## References

- [Official Rust documentation · ch04 01 what is ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

---

[← Previous: Garbage collection](<../06 - Garbage collection/README.md>) | [Back to Memory Management Fundamentals](<../README.md>) | [Next: Memory leaks →](<../08 - Memory leaks/README.md>)
