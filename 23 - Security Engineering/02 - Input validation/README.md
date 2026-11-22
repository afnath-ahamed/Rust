# Input validation

Validate external data at trust boundaries before converting it to a domain type. Enforce format, length, ranges, and cardinality. Use limits appropriate to the amount of work triggered.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn port(input: &str) -> Option<u16> {
    if input.is_empty() || input.len() > 5 || !input.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    input.parse::<u16>().ok().filter(|&value| value != 0)
}
assert_eq!(port("443"), Some(443));
assert_eq!(port("65536"), None);
```

## 2. Common mistake

Parsing confirms representability, while validation checks whether the value fits application policy.

## Practice

Add tests for a leading sign, zero, too much input, and the maximum legal port.

## References

- [rustsec.org](https://rustsec.org/)

---

[← Previous: Memory safety guarantees](<../01 - Memory safety guarantees/README.md>) | [Back to Security Engineering](<../README.md>) | [Next: Secure coding practices →](<../03 - Secure coding practices/README.md>)
