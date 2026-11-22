# Enums

An enum defines a fixed set of alternatives. Variants can be fieldless, positional, or named-field values. Pattern matching handles the data that belongs to each alternative.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    enum State {
        Waiting,
        Running { job: u32 },
    }
    let states = [State::Waiting, State::Running { job: 7 }];
    assert!(matches!(states[1], State::Running { job: 7 }));
}
```

## 2. Common mistake

Replacing states with unrelated booleans often permits impossible combinations.

## Practice

Model a download as waiting, complete with size, or failed with an error description.

## References

- [Official Rust documentation · ch05 00 structs](https://doc.rust-lang.org/book/ch05-00-structs.html)

---

[← Previous: Implementation blocks (impl)](<../06 - Implementation blocks (impl)/README.md>) | [Back to Structs and Enums](<../README.md>) | [Next: `Option<T>` →](<../08 - Option/README.md>)
