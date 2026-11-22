# Mocking

Mocking substitutes a controlled collaborator so tests can assert behavior around errors, time, or storage. A small fake behind a trait can cover domain behavior with less setup.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
trait Clock {
    fn seconds(&self) -> u64;
}
struct Fixed(u64);
impl Clock for Fixed {
    fn seconds(&self) -> u64 {
        self.0
    }
}
fn expired(clock: &impl Clock, until: u64) -> bool {
    clock.seconds() >= until
}
assert!(expired(&Fixed(10), 10));
```

## 2. Common mistake

Mocking every internal method can make tests sensitive to harmless refactors.

## Practice

Test two expiry boundaries with the same fake clock.

## References

- [Official Rust documentation · ch11 00 testing](https://doc.rust-lang.org/book/ch11-00-testing.html)

---

[← Previous: Property-based testing](<../04 - Property-based testing/README.md>) | [Back to Testing and Debugging](<../README.md>) | [Next: Fuzz testing →](<../06 - Fuzz testing/README.md>)
