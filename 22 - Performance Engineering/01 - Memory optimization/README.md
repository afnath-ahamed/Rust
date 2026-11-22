# Memory optimization

Memory optimization begins by measuring resident memory, allocation volume, and the size of common values. Change representation only when it reduces the relevant cost under realistic inputs.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
use std::mem::size_of;
assert_eq!(size_of::<[u64; 8]>(), 64);
assert_eq!(size_of::<[u32; 8]>(), 32);
```

## 2. Common mistake

size_of reports inline size. A vector or string can own additional memory that this measurement excludes.

## Practice

Compare two representations for a dense counter table and test whether a narrower integer can overflow.

## References

- [Official Rust documentation · profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)

---

[← Previous: Software Architecture](<../../21 - Software Architecture/README.md>) | [Back to Performance Engineering](<../README.md>) | [Next: CPU optimization →](<../02 - CPU optimization/README.md>)
