# Queues

A queue returns the oldest item first. VecDeque provides push_back and pop_front without shifting every stored element.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    use std::collections::VecDeque;
    let mut tasks = VecDeque::from(["first", "second"]);
    tasks.push_back("third");
    assert_eq!(tasks.pop_front(), Some("first"));
}
```

## 2. Common mistake

Removing index zero from a Vec repeatedly shifts the remaining elements.

## Practice

Implement a fixed-size recent-history queue and test eviction.

## Related lesson

[VecDeque](<../../08 - Collections and Strings/08 - VecDeque/README.md>)

## References

- [Official Rust documentation · collections](https://doc.rust-lang.org/std/collections/)

---

[← Previous: Stacks](<../05 - Stacks/README.md>) | [Back to Data Structures and Algorithms](<../README.md>) | [Next: Trees →](<../07 - Trees/README.md>)
