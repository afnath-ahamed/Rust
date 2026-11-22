# Benchmark analysis

Benchmark analysis compares distributions, not single measurements. Use warm-up, repeated runs, representative sizes, and matching build settings. Record latency percentiles or throughput as appropriate.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
Before batch (ms): 10, 11, 11, 12, 19
After batch (ms):   9, 10, 10, 10, 11
Compare median and tail separately, then repeat with realistic traffic.
```

## 2. Common mistake

Avoid claiming improvement if the effect is within run-to-run variation.

## Practice

Report median and maximum for two groups, then describe which additional measurements are needed.

## References

- [Official Rust documentation · profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)

---

[← Previous: Flame graphs](<../06 - Flame graphs/README.md>) | [Back to Performance Engineering](<../README.md>) | [Next: SIMD →](<../08 - SIMD/README.md>)
