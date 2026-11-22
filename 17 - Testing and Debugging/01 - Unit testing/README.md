# Unit testing

Unit tests exercise small pieces of logic, often beside the code they cover. Rust's test harness discovers functions marked with test. Arrange the input, perform the operation, and assert the behavior.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```rust
fn clamp(value: i32) -> i32 {
    value.clamp(0, 100)
}
assert_eq!(clamp(-1), 0);
assert_eq!(clamp(105), 100);
```

```shell
cargo test
```

## 2. Common mistake

Asserting output alone may miss boundary rules. Include empty, minimal, and maximum cases where relevant.

## Practice

Move the assertions into a cfg(test) module with distinct test cases.

## References

- [Official Rust documentation · ch11 00 testing](https://doc.rust-lang.org/book/ch11-00-testing.html)

---

[← Previous: Advanced Rust](<../../16 - Advanced Rust/README.md>) | [Back to Testing and Debugging](<../README.md>) | [Next: Integration testing →](<../02 - Integration testing/README.md>)
