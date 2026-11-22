# `Cow<T>`

Cow can store borrowed or owned data and clone on mutation when needed. It is useful when most inputs can be returned as borrowed views.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
use std::borrow::Cow;
fn clean(input: &str) -> Cow<'_, str> {
    if input.contains('-') {
        Cow::Owned(input.replace('-', "_"))
    } else {
        Cow::Borrowed(input)
    }
}
assert!(matches!(clean("a"), Cow::Borrowed(_)));
assert_eq!(clean("a-b"), "a_b");
```

## 2. Common mistake

Calling into_owned on a borrowed value forces owned data to be produced.

## Practice

Extend clean to support trimming without allocating when the result can be a subslice.

## References

- [Official Rust documentation · ch15 00 smart pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)

---

[← Previous: `Weak<T>`](<../06 - Weak/README.md>) | [Back to Smart Pointers](<../README.md>) | [Next: Deref trait →](<../08 - Deref trait/README.md>)
