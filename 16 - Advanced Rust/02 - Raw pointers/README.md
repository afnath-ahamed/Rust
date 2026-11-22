# Raw pointers

Raw pointers may be created without establishing all reference guarantees. A raw dereference needs a valid, suitably aligned pointer to initialized storage and access consistent with the aliasing rules.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let number = 42;
    let pointer = std::ptr::addr_of!(number);
    // SAFETY: number is live, aligned, and accessed here only for reading.
    assert_eq!(unsafe { *pointer }, 42);
}
```

## 2. Common mistake

A non-null pointer may still be dangling or improperly aligned.

## Practice

Annotate the live owner and introduce a safe scoped wrapper around the read.

## Related lesson

[Pointers](<../../03 - Memory Management Fundamentals/05 - Pointers/README.md>)

## References

- [Official Rust documentation · ch20 00 advanced features](https://doc.rust-lang.org/book/ch20-00-advanced-features.html)

---

[← Previous: Unsafe Rust](<../01 - Unsafe Rust/README.md>) | [Back to Advanced Rust](<../README.md>) | [Next: Unsafe functions →](<../03 - Unsafe functions/README.md>)
