# Iterators

An iterator yields successive values through next. Borrow elements with iter, mutably borrow with iter_mut, and use into_iter when ownership transfer is intended for an owned collection.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let numbers = vec![1, 2, 3];
    let mut values = numbers.iter();
    assert_eq!(values.next(), Some(&1));
    assert_eq!(numbers.len(), 3);
}
```

## 2. Common mistake

The iterator's Item type determines whether a step yields an owned value or a reference.

## Practice

Rewrite the example to consume a Vec of String and explain the new Item type.

## References

- [Official Rust documentation · ch13 00 functional features](https://doc.rust-lang.org/book/ch13-00-functional-features.html)

---

[← Previous: `Fn / FnMut / FnOnce`](<../02 - Fn, FnMut and FnOnce/README.md>) | [Back to Functional Programming](<../README.md>) | [Next: Iterator adapters →](<../04 - Iterator adapters/README.md>)
