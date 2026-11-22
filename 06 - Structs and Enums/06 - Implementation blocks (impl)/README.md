# Implementation blocks (impl)

An inherent impl defines methods and associated items for a type. A trait impl uses impl Trait for Type to fulfill a particular interface. A type may have multiple inherent impl blocks.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    struct Report(u32);
    impl Report {
        fn count(&self) -> u32 {
            self.0
        }
    }
    impl Report {
        fn is_empty(&self) -> bool {
            self.0 == 0
        }
    }
    let report = Report(0);
    assert!(report.is_empty());
    assert_eq!(report.count(), 0);
}
```

## 2. Common mistake

Split impl blocks can group concerns, but duplicate inherent member names are invalid.

## Practice

Group initialization and query methods in different impl blocks, then call both.

## References

- [Official Rust documentation · ch05 00 structs](https://doc.rust-lang.org/book/ch05-00-structs.html)

---

[← Previous: Associated functions](<../05 - Associated functions/README.md>) | [Back to Structs and Enums](<../README.md>) | [Next: Enums →](<../07 - Enums/README.md>)
