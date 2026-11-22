# Memory safety guarantees

Sound safe Rust prevents classes of invalid memory access and data races. The guarantees depend on the soundness of unsafe code underlying safe interfaces. Authorization and application invariants require independent controls.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let readings = [5, 8, 13];
    assert_eq!(readings.get(8), None);
}
```

## 2. Common mistake

A memory-safe program can still process untrusted input incorrectly or exhaust resources.

## Practice

Choose a parsing boundary and list separate checks for bounds, validity, and resource limits.

## References

- [rustsec.org](https://rustsec.org/)

---

[← Previous: Performance Engineering](<../../22 - Performance Engineering/README.md>) | [Back to Security Engineering](<../README.md>) | [Next: Input validation →](<../02 - Input validation/README.md>)
