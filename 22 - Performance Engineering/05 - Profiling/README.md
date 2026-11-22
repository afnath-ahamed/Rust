# Profiling

A profiler attributes resource use to observed execution. Choose CPU sampling, allocation profiling, or lock analysis based on the symptom and capture a representative workload.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```toml
[profile.release]
debug = "line-tables-only"
```

```shell
cargo build --release
```

## 2. Common mistake

Choose a profiler that supports your platform and keep symbols available. A debug build can produce a very different profile.

## Practice

Record a profile before changing code and identify the hottest stack and the operation it represents.

## Related lesson

[Benchmarking](<../../17 - Testing and Debugging/07 - Benchmarking/README.md>)

## References

- [Official Rust documentation · profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)

---

[← Previous: Cache locality](<../04 - Cache locality/README.md>) | [Back to Performance Engineering](<../README.md>) | [Next: Flame graphs →](<../06 - Flame graphs/README.md>)
