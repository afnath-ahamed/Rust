# Rayon

Rayon offers data parallelism through parallel iterators and task splitting. Work independently per item and use suitable associative reductions. Parallel order need not match a sequential evaluation order.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
cargo add rayon
```

```rust no_run
fn main() {
    use rayon::prelude::*;
    let total: u64 = (1_u64..=1000).into_par_iter().map(|n| n * n).sum();
    assert_eq!(total, 333_833_500);
}
```

## 2. Common mistake

Measure realistic inputs before parallelizing and avoid shared locks inside every iteration.

## Practice

Compare serial and parallel processing for cheap items and CPU-heavy items.

## References

- [docs.rs · rayon](https://docs.rs/rayon/latest/rayon/)
- [Official Rust documentation · ch16 00 concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

---

[← Previous: Deadlocks](<../13 - Deadlocks/README.md>) | [Back to Concurrency and Parallelism](<../README.md>) | [Next: Asynchronous Programming →](<../../15 - Asynchronous Programming/README.md>)
