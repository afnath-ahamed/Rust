# Trait bounds

A trait bound specifies an operation a generic parameter must support. Combine bounds using + and use a where clause to keep long signatures readable.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn larger<T>(left: T, right: T) -> T
where
    T: Ord,
{
    if left >= right { left } else { right }
}
assert_eq!(larger(7, 4), 7);
```

## 2. Common mistake

Choose the weakest bound that models the contract. Ord excludes floats because NaN prevents a total ordering.

## Practice

Change the function to return Option<T> with PartialOrd and test a NaN input.

## References

- [Official Rust documentation · ch10 00 generics](https://doc.rust-lang.org/book/ch10-00-generics.html)

---

[← Previous: Trait implementation](<../05 - Trait implementation/README.md>) | [Back to Generics, Traits and Lifetimes](<../README.md>) | [Next: Default implementations →](<../07 - Default implementations/README.md>)
