# `Result<T, E>`

Result<T, E> represents success with Ok(T) or failure with Err(E). Treat it as a data type here. Error policy and propagation belong in Error Handling.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn verified(number: i32) -> Result<i32, &'static str> {
    if number >= 0 {
        Ok(number)
    } else {
        Err("negative value")
    }
}
assert_eq!(verified(-1), Err("negative value"));
```

## 2. Common mistake

Returning a boolean would lose the reason a failed value was rejected.

## Practice

Return a richer error enum with separate variants for negative and over-limit values.

## Related lesson

[Result](<../../09 - Error Handling/04 - Result/README.md>)

## References

- [Official Rust documentation · ch05 00 structs](https://doc.rust-lang.org/book/ch05-00-structs.html)

---

[← Previous: `Option<T>`](<../08 - Option/README.md>) | [Back to Structs and Enums](<../README.md>) | [Next: Pattern Matching →](<../../07 - Pattern Matching/README.md>)
