# Result

Result makes success and failure part of an API. Use match when each path needs its own behavior, map when transforming success, and map_err when translating the error.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let doubled = "21".parse::<u32>().map(|n| n * 2);
    assert_eq!(doubled.ok(), Some(42));
}
```

## 2. Common mistake

Turning Result into Option with ok discards error detail permanently.

## Practice

Add an error translation that preserves the operation name in a readable message.

## Related lesson

[`Result<T, E>`](<../../06 - Structs and Enums/09 - Result/README.md>)

## References

- [Official Rust documentation · ch09 00 error handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

---

[← Previous: Panic](<../03 - Panic/README.md>) | [Back to Error Handling](<../README.md>) | [Next: Option →](<../05 - Option/README.md>)
