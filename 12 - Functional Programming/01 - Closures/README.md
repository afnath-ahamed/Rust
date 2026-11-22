# Closures

A closure is a value containing callable behavior and, when needed, captured values. The compiler infers capture modes from how the closure uses its environment.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let offset = 3;
    let translate = |number| number + offset;
    assert_eq!(translate(7), 10);
}
```

## 2. Common mistake

move captures values by value, but the closure may still implement Fn when calls do not consume or mutate captured state.

## Practice

Build a closure borrowing a local setting, then use move to transfer an owned setting into it.

## Related lesson

[Trait bounds](<../../11 - Generics, Traits and Lifetimes/06 - Trait bounds/README.md>)

## References

- [Official Rust documentation · ch13 00 functional features](https://doc.rust-lang.org/book/ch13-00-functional-features.html)

---

[← Previous: Generics, Traits and Lifetimes](<../../11 - Generics, Traits and Lifetimes/README.md>) | [Back to Functional Programming](<../README.md>) | [Next: `Fn / FnMut / FnOnce` →](<../02 - Fn, FnMut and FnOnce/README.md>)
