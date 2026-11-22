# Zero-cost abstractions

Rust's iterator abstractions often optimize to code comparable to hand-written loops. The result depends on data layout, allocation, build settings, and the operation itself.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let input = [1u32, 2, 3, 4];
    let pipeline: u32 = input.into_iter().filter(|n| n % 2 == 0).sum();
    let mut manual = 0;
    for n in input {
        if n % 2 == 0 {
            manual += n;
        }
    }
    assert_eq!(pipeline, manual);
}
```

## 2. Common mistake

Collecting intermediate vectors introduces work that a composed lazy pipeline can avoid.

## Practice

Benchmark a single pipeline and an equivalent loop under the same optimized profile.

## References

- [Official Rust documentation · ch13 00 functional features](https://doc.rust-lang.org/book/ch13-00-functional-features.html)

---

[← Previous: Lazy evaluation](<../09 - Lazy evaluation/README.md>) | [Back to Functional Programming](<../README.md>) | [Next: Smart Pointers →](<../../13 - Smart Pointers/README.md>)
