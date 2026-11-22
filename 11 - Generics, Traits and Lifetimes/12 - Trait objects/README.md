# Trait objects

A trait object such as Box<dyn Trait> erases the concrete type while keeping supported trait operations. It can hold different implementations in one collection.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    trait Job {
        fn id(&self) -> u8;
    }
    struct Read;
    struct Write;
    impl Job for Read {
        fn id(&self) -> u8 {
            1
        }
    }
    impl Job for Write {
        fn id(&self) -> u8 {
            2
        }
    }
    let jobs: Vec<Box<dyn Job>> = vec![Box::new(Read), Box::new(Write)];
    assert_eq!(jobs.iter().map(|job| job.id()).collect::<Vec<_>>(), [1, 2]);
}
```

## 2. Common mistake

The trait must be dyn-compatible. Generic methods usually need Self: Sized to remain outside the trait object's callable interface.

## Practice

Change the collection to borrowed trait objects and keep the original owners alive.

## References

- [Official Rust documentation · ch10 00 generics](https://doc.rust-lang.org/book/ch10-00-generics.html)

---

[← Previous: Static lifetime](<../11 - Static lifetime/README.md>) | [Back to Generics, Traits and Lifetimes](<../README.md>) | [Next: Static vs Dynamic dispatch →](<../13 - Static vs Dynamic dispatch/README.md>)
