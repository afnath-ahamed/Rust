# Stack memory

A function call conceptually gets a stack frame for its local working state. Stack space is reclaimed as calls complete. Optimizers may inline calls or keep values in registers, so source declarations do not promise a physical location.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn bump(value: i32) -> i32 {
    let next = value + 1;
    next
}
assert_eq!(bump(41), 42);
```

## 2. Common mistake

Deep recursion can consume the thread stack even when the function allocates no heap data.

## Practice

Rewrite a recursive countdown as a loop and explain what happens to call depth.

## References

- [Official Rust documentation · ch04 01 what is ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

---

[← Previous: Control Flow](<../../02 - Programming Fundamentals/12 - Control Flow/README.md>) | [Back to Memory Management Fundamentals](<../README.md>) | [Next: Heap memory →](<../02 - Heap memory/README.md>)
