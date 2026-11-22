# Static lifetime

A &'static reference is valid for the entire program, as with a string literal. A T: 'static bound means T contains no borrowed data with a shorter required lifetime. An owned String can satisfy that bound and still drop normally.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    fn valid_for_static_bound<T: 'static>(value: T) -> T {
        value
    }
    let owned = valid_for_static_bound(String::from("temporary owner"));
    let literal: &'static str = "program text";
    assert_eq!(literal.len(), 12);
    drop(owned);
}
```

## 2. Common mistake

A 'static bound does not mean every instance lives forever.

## Practice

Try passing a reference to a local String to the generic function and explain the error.

## Related lesson

[Dangling references](<../../05 - Borrowing and References/06 - Dangling references/README.md>)

## References

- [Official Rust documentation · ch10 00 generics](https://doc.rust-lang.org/book/ch10-00-generics.html)

---

[← Previous: Lifetime elision](<../10 - Lifetime elision/README.md>) | [Back to Generics, Traits and Lifetimes](<../README.md>) | [Next: Trait objects →](<../12 - Trait objects/README.md>)
