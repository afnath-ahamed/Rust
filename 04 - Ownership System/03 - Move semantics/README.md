# Move semantics

Moving transfers ownership without requiring a deep clone of the value's separately allocated data. Assignment, by-value arguments, and returns are common move points.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let original = vec![1, 2];
    let mut moved = original;
    moved.push(3);
    assert_eq!(moved, [1, 2, 3]);
}
```

## 2. Common mistake

Do not equate a language-level move with a guarantee that bytes are physically relocated in a particular way.

## Practice

Pass moved to a function taking Vec<i32> and identify the owner in the function.

## References

- [Official Rust documentation · ch04 01 what is ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

---

[← Previous: Variable scope](<../02 - Variable scope/README.md>) | [Back to Ownership System](<../README.md>) | [Next: Copy semantics →](<../04 - Copy semantics/README.md>)
