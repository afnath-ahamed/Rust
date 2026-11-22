# Match expressions

A match tests patterns in order and executes the first applicable arm. Its arms can destructure input data and must yield compatible result types when the match produces a value.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let reading = Some(7);
    let text = match reading {
        Some(n) => format!("{n} units"),
        None => String::from("missing"),
    };
    assert_eq!(text, "7 units");
}
```

## 2. Common mistake

A catch-all arm early in the match can conceal a more specific case.

## Practice

Match three variants of an enum and give each one a distinct description.

## Related lesson

[Enums](<../../06 - Structs and Enums/07 - Enums/README.md>)

## References

- [Official Rust documentation · ch19 00 patterns](https://doc.rust-lang.org/book/ch19-00-patterns.html)

---

[← Previous: Structs and Enums](<../../06 - Structs and Enums/README.md>) | [Back to Pattern Matching](<../README.md>) | [Next: If let →](<../02 - If let/README.md>)
