# Clone

Clone is an explicit duplication operation with type-specific cost and semantics. Cloning a String copies its text. Cloning an Rc shares the same allocation and increments its strong count.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let first = String::from("cat");
    let mut second = first.clone();
    second.push('s');
    assert_eq!((first.as_str(), second.as_str()), ("cat", "cats"));
}
```

## 2. Common mistake

Adding clone everywhere can hide a poorly chosen ownership boundary and introduce allocations.

## Practice

Change a function that clones a read-only String to accept a borrowed string slice.

## References

- [Official Rust documentation · ch04 01 what is ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

---

[← Previous: Copy semantics](<../04 - Copy semantics/README.md>) | [Back to Ownership System](<../README.md>) | [Next: Ownership with functions →](<../06 - Ownership with functions/README.md>)
