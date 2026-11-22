# Stacks

A stack returns the most recently pushed item first. Vec supports stack behavior through push and pop. Use the Option from pop to handle empty input.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let mut history = Vec::new();
    history.push("edit 1");
    history.push("edit 2");
    assert_eq!(history.pop(), Some("edit 2"));
}
```

## 2. Common mistake

Recursive traversal also uses a call stack. An explicit stack can avoid excessive recursion depth.

## Practice

Check whether a string of parentheses is balanced using a stack.

## References

- [Official Rust documentation · collections](https://doc.rust-lang.org/std/collections/)

---

[← Previous: Linked lists](<../04 - Linked lists/README.md>) | [Back to Data Structures and Algorithms](<../README.md>) | [Next: Queues →](<../06 - Queues/README.md>)
