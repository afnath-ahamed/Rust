# Cache locality

Spatial locality improves when successive accesses reach nearby memory. Contiguous arrays and column layouts can help scans that use a subset of each record.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let readings = [1.0f64, 2.0, 3.0];
    assert_eq!(readings.iter().sum::<f64>(), 6.0);
}
```

## 2. Common mistake

Changing from an array of records to separate columns can slow code that needs every field together.

## Practice

Compare row-based data with column-based data when summing one field and when reading all fields.

## References

- [Official Rust documentation · profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)

---

[← Previous: Allocation optimization](<../03 - Allocation optimization/README.md>) | [Back to Performance Engineering](<../README.md>) | [Next: Profiling →](<../05 - Profiling/README.md>)
