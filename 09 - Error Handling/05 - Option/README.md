# Option

Option models a missing value with no failure reason. Use ok_or or ok_or_else to attach a reason when absence becomes an error at the caller's boundary.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let names = ["Ana", "Ben"];
    let selected = names.get(4).ok_or("name index out of range");
    assert_eq!(selected, Err("name index out of range"));
}
```

## 2. Common mistake

Do not use None to collapse network or storage failures when their distinction affects callers.

## Practice

Return Ok(None) when a record is absent and Err for an unavailable storage backend.

## Related lesson

[`Option<T>`](<../../06 - Structs and Enums/08 - Option/README.md>)

## References

- [Official Rust documentation · ch09 00 error handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

---

[← Previous: Result](<../04 - Result/README.md>) | [Back to Error Handling](<../README.md>) | [Next: Unwrap and Expect →](<../06 - Unwrap and Expect/README.md>)
