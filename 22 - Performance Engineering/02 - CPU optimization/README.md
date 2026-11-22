# CPU optimization

Improve CPU use by finding where time goes, reducing wasted work, and considering algorithmic changes before local tuning. Compare the same inputs and outputs under both versions.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let precomputed = [0, 1, 4, 9, 16];
    let index = 3;
    assert_eq!(precomputed[index], index * index);
}
```

## 2. Common mistake

Precomputation can trade CPU for memory or shift work to initialization. Measure the full workload.

## Practice

Find one repeated computation and measure both caching and a simpler algorithmic change.

## References

- [Official Rust documentation · profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)

---

[← Previous: Memory optimization](<../01 - Memory optimization/README.md>) | [Back to Performance Engineering](<../README.md>) | [Next: Allocation optimization →](<../03 - Allocation optimization/README.md>)
