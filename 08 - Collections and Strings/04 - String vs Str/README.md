# String vs &str

String owns growable UTF-8 text. &str is a borrowed UTF-8 view. Accept &str to let both literals and owned strings serve as read-only inputs.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    fn clean(label: &str) -> &str {
        label.trim()
    }
    let input = String::from("  Rust  ");
    assert_eq!(clean(&input), "Rust");
    assert_eq!(clean("  hi "), "hi");
}
```

## 2. Common mistake

Creating a new String for a function that only needs a borrow wastes an allocation.

## Practice

Add a function whose result must be independent of its input and explain why it returns String.

## Related lesson

[Slices](<../../05 - Borrowing and References/08 - Slices/README.md>)

## References

- [Official Rust documentation · collections](https://doc.rust-lang.org/std/collections/)

---

[← Previous: Vectors](<../03 - Vectors/README.md>) | [Back to Collections and Strings](<../README.md>) | [Next: HashMap →](<../05 - HashMap/README.md>)
