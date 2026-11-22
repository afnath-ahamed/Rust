# Dependency injection

Dependency injection provides collaborators from the composition boundary. Inject clocks, repositories, or configuration so core behavior is testable with deterministic substitutes.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
trait Clock {
    fn now(&self) -> u64;
}
fn remaining(clock: &impl Clock, expiry: u64) -> u64 {
    expiry.saturating_sub(clock.now())
}
struct Fixed;
impl Clock for Fixed {
    fn now(&self) -> u64 {
        5
    }
}
assert_eq!(remaining(&Fixed, 8), 3);
```

## 2. Common mistake

Constructing a system clock inside every use case makes expiry tests difficult to reproduce.

## Practice

Override the clock and cover exact expiry and already-expired cases.

## References

- [Official Rust documentation · ch07 00 managing growing projects with packages crates and modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

[← Previous: Design patterns](<../02 - Design patterns/README.md>) | [Back to Software Architecture](<../README.md>) | [Next: Layered architecture →](<../04 - Layered architecture/README.md>)
