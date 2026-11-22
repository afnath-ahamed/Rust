# SIMD

SIMD performs similar operations over lanes of data. A simple loop can autovectorize, and architecture intrinsics require explicit feature and fallback choices. Validate results and benchmark supported targets.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let left = [1u32, 3, 5, 7];
    let right = [2u32, 4, 6, 8];
    let sum = std::array::from_fn::<_, 4, _>(|i| left[i] + right[i]);
    assert_eq!(sum, [3, 7, 11, 15]);
}
```

## 2. Common mistake

Source code that resembles SIMD is not proof the compiler emitted SIMD. Inspect optimized code or a relevant performance counter.

## Practice

Compare an ordinary loop against an optimized implementation with deterministic correctness checks.

## References

- [Official Rust documentation · profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)

---

[← Previous: Benchmark analysis](<../07 - Benchmark analysis/README.md>) | [Back to Performance Engineering](<../README.md>) | [Next: Release optimization →](<../09 - Release optimization/README.md>)
