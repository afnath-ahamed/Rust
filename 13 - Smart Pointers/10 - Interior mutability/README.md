# Interior mutability

Interior mutability permits selected updates through shared references while preserving Rust's safety invariants. Cell, RefCell, mutexes, and atomics apply different enforcement models.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    use std::cell::Cell;
    struct Reads(Cell<u32>);
    impl Reads {
        fn record(&self) {
            self.0.set(self.0.get() + 1);
        }
    }
    let stats = Reads(Cell::new(0));
    stats.record();
    assert_eq!(stats.0.get(), 1);
}
```

## 2. Common mistake

Do not choose an interior-mutable type until you know whether access is single-threaded or shared across threads.

## Practice

Compare a Cell counter with an AtomicU32 counter for two consumers.

## References

- [Official Rust documentation · ch15 00 smart pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)

---

[← Previous: Drop trait](<../09 - Drop trait/README.md>) | [Back to Smart Pointers](<../README.md>) | [Next: Reference cycles →](<../11 - Reference cycles/README.md>)
