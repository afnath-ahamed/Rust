# Unsafe functions

An unsafe function puts preconditions on callers. Document them in a Safety section and use explicit unsafe blocks for operations inside the body. Safe wrappers must establish every precondition themselves.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    /// Read one live, aligned, initialized u32 through a pointer.
    ///
    /// # Safety
    /// The pointer must be valid for a read and satisfy Rust's aliasing rules.
    unsafe fn load(pointer: *const u32) -> u32 {
        // SAFETY: guaranteed by the caller.
        unsafe { *pointer }
    }
    let value = 7;
    // SAFETY: value stays live and is only read.
    assert_eq!(unsafe { load(&value) }, 7);
}
```

## 2. Common mistake

Marking a function unsafe describes a caller obligation. Validate any inputs that a safe caller could supply before invoking it.

## Practice

Show how a wrapper accepting &u32 establishes the required pointer guarantees.

## References

- [Official Rust documentation · ch20 00 advanced features](https://doc.rust-lang.org/book/ch20-00-advanced-features.html)

---

[← Previous: Raw pointers](<../02 - Raw pointers/README.md>) | [Back to Advanced Rust](<../README.md>) | [Next: Advanced traits →](<../04 - Advanced traits/README.md>)
