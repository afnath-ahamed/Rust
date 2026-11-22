# If let

Use if let when one pattern is the interesting case. Bindings from the pattern are available in the successful branch. An optional else handles remaining cases together.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let label = Some("rust");
    let mut seen = false;
    if let Some(name) = label {
        seen = name == "rust";
    }
    assert!(seen);
}
```

## 2. Common mistake

Use match when several alternatives need distinct handling.

## Practice

Add an else that records a fallback for a missing label.

## References

- [Official Rust documentation · ch19 00 patterns](https://doc.rust-lang.org/book/ch19-00-patterns.html)

---

[← Previous: Match expressions](<../01 - Match expressions/README.md>) | [Back to Pattern Matching](<../README.md>) | [Next: While let →](<../03 - While let/README.md>)
