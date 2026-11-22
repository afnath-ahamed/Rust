# Match guards

A match guard adds a boolean condition after a pattern. If the condition fails, matching continues. Include an unguarded fallback for the relevant variants.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let input = Some(11);
    let class = match input {
        Some(n) if n > 10 => "high",
        Some(_) => "normal",
        None => "missing",
    };
    assert_eq!(class, "high");
}
```

## 2. Common mistake

Guard conditions do not establish exhaustiveness for all values under their pattern.

## Practice

Add a guarded negative case and test the exact endpoints.

## References

- [Official Rust documentation · ch19 00 patterns](https://doc.rust-lang.org/book/ch19-00-patterns.html)

---

[← Previous: Destructuring](<../05 - Destructuring/README.md>) | [Back to Pattern Matching](<../README.md>) | [Next: Exhaustive matching →](<../07 - Exhaustive matching/README.md>)
