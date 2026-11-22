# Memory leaks

A leak occurs when resources remain allocated after they are no longer useful. Safe code can still leak memory, for example through long-lived caches or reference-counting cycles.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    use std::collections::HashMap;
    let mut cache = HashMap::from([("old", String::from("data"))]);
    cache.remove("old");
    assert!(cache.is_empty());
}
```

## 2. Common mistake

A reachable cache can grow indefinitely even when memory safety holds.

## Practice

Design a maximum-entry policy and state when the oldest value would be evicted.

## Related lesson

[Interior mutability](<../../13 - Smart Pointers/10 - Interior mutability/README.md>)

## References

- [Official Rust documentation · ch04 01 what is ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

---

[← Previous: Dangling pointers](<../07 - Dangling pointers/README.md>) | [Back to Memory Management Fundamentals](<../README.md>) | [Next: Memory safety →](<../09 - Memory safety/README.md>)
