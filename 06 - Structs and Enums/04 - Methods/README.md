# Methods

A method declares a receiver such as &self, &mut self, or self. The receiver communicates whether the method borrows, mutably borrows, or consumes the instance.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    struct Counter(u32);
    impl Counter {
        fn add(&mut self) {
            self.0 += 1;
        }
        fn value(&self) -> u32 {
            self.0
        }
    }
    let mut c = Counter(0);
    c.add();
    assert_eq!(c.value(), 1);
}
```

## 2. Common mistake

An accessor generally needs a shared receiver when it only reads.

## Practice

Add a method that consumes Counter and returns its final value.

## Related lesson

[Traits](<../../11 - Generics, Traits and Lifetimes/04 - Traits/README.md>)

## References

- [Official Rust documentation · ch05 00 structs](https://doc.rust-lang.org/book/ch05-00-structs.html)

---

[← Previous: Unit-like structs](<../03 - Unit-like structs/README.md>) | [Back to Structs and Enums](<../README.md>) | [Next: Associated functions →](<../05 - Associated functions/README.md>)
