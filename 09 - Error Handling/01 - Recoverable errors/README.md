# Recoverable errors

Expected failures such as invalid input belong in Result. At an application boundary, distinguish failures the caller can correct from failures that need escalation or retry.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    match "bad".parse::<u16>() {
        Ok(port) => println!("port {port}"),
        Err(_) => println!("Enter a valid port number"),
    }
}
```

## 2. Common mistake

A retry makes sense only when the failure could change and replaying the operation is safe.

## Practice

Extend a parser to show a different message for invalid syntax and a valid but disallowed value.

## References

- [Official Rust documentation · ch09 00 error handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

---

[← Previous: Collections and Strings](<../../08 - Collections and Strings/README.md>) | [Back to Error Handling](<../README.md>) | [Next: Unrecoverable errors →](<../02 - Unrecoverable errors/README.md>)
