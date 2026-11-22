# Allocation optimization

Heap allocations can add cost in tight loops. Reserve when size is known, borrow temporary views, or reuse an existing buffer. Observe allocations and elapsed time before deciding.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let mut buffer = Vec::with_capacity(64);
    for batch in [[1, 2], [3, 4]] {
        buffer.clear();
        buffer.extend(batch);
        assert_eq!(buffer.len(), 2);
    }
    assert!(buffer.capacity() >= 64);
}
```

## 2. Common mistake

A reused buffer can retain its largest capacity. Establish limits for untrusted or atypically large requests.

## Practice

Measure allocations for fresh versus reused buffers at small and large batch sizes.

## References

- [Official Rust documentation · profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)

---

[← Previous: CPU optimization](<../02 - CPU optimization/README.md>) | [Back to Performance Engineering](<../README.md>) | [Next: Cache locality →](<../04 - Cache locality/README.md>)
