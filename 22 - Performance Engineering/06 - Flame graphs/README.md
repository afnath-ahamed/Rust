# Flame graphs

A flame graph aggregates sampled stack frames. Width represents the count or weight of samples, and vertical placement represents call ancestry. Horizontal position usually does not represent elapsed time.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
Root: import workload — 100 sampled stacks
  Decode input — 65
  Validate rows — 25
  Emit output — 10
```

## 2. Common mistake

A wide parent may include work charged to its children. Inspect leaf frames and verify the measured resource.

## Practice

Read two profiles with the same workload and explain which hot path actually changed.

## References

- [Official Rust documentation · profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)

---

[← Previous: Profiling](<../05 - Profiling/README.md>) | [Back to Performance Engineering](<../README.md>) | [Next: Benchmark analysis →](<../07 - Benchmark analysis/README.md>)
