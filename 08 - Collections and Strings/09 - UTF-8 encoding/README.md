# UTF-8 encoding

UTF-8 encodes Unicode scalar values in one to four bytes. len counts bytes and chars counts scalar values. User-perceived characters can span multiple scalars, so grapheme-aware logic needs a suitable library.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let label = "café";
    assert_eq!(label.len(), 5);
    assert_eq!(label.chars().count(), 4);
    assert_eq!(label.get(..4), None);
}
```

## 2. Common mistake

Byte offsets are safe slice endpoints only at UTF-8 character boundaries.

## Practice

Compare byte, scalar, and visible-character counts for text with a combining accent.

## Related lesson

[Slices](<../../05 - Borrowing and References/08 - Slices/README.md>)

## References

- [Official Rust documentation · collections](https://doc.rust-lang.org/std/collections/)

---

[← Previous: VecDeque](<../08 - VecDeque/README.md>) | [Back to Collections and Strings](<../README.md>) | [Next: Error Handling →](<../../09 - Error Handling/README.md>)
