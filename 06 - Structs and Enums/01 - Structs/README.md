# Structs

A named-field struct combines related data under one type. Instantiate it with field names and use dot access or destructuring to inspect it. Field visibility can protect validity rules across modules.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    struct Measure {
        width: u32,
        height: u32,
    }
    let size = Measure {
        width: 3,
        height: 4,
    };
    assert_eq!(size.width * size.height, 12);
}
```

## 2. Common mistake

Making every field public lets callers construct states your methods may assume are impossible.

## Practice

Design a Rectangle constructor that checks an application rule about zero sizes.

## References

- [Official Rust documentation · ch05 00 structs](https://doc.rust-lang.org/book/ch05-00-structs.html)

---

[← Previous: Borrowing and References](<../../05 - Borrowing and References/README.md>) | [Back to Structs and Enums](<../README.md>) | [Next: Tuple structs →](<../02 - Tuple structs/README.md>)
