# Return values and ownership

Returning a value transfers it to the caller. This is a clean way for functions to construct owned data. Returning a borrow needs a valid source that still lives in the caller.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    fn make_label(number: u32) -> String {
        format!("item-{number}")
    }
    let label = make_label(4);
    assert_eq!(label, "item-4");
}
```

## 2. Common mistake

Returning &str does not extend the life of an owned String created inside the function.

## Practice

Write a function that takes a String, appends a suffix, and returns ownership to the caller.

## References

- [Official Rust documentation · ch04 01 what is ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

---

[← Previous: Ownership with functions](<../06 - Ownership with functions/README.md>) | [Back to Ownership System](<../README.md>) | [Next: Drop →](<../08 - Drop/README.md>)
