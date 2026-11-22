# Release optimization

Release profiles control runtime speed, optimization size, compiler work, and diagnostics. LTO and codegen unit counts can change those trade-offs. Profile-guided optimization needs representative training traffic.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```toml
[profile.release]
lto = "thin"
codegen-units = 1
```

```shell
cargo build --release
```

## 2. Common mistake

Measure build time, binary size, and throughput separately. The fastest profile for one workload may not win another.

## Practice

Compare a default optimized build with thin LTO and keep a reproducible measurement log.

## References

- [Official Rust documentation · profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)

---

[← Previous: SIMD](<../08 - SIMD/README.md>) | [Back to Performance Engineering](<../README.md>) | [Next: Binary size optimization →](<../10 - Binary size optimization/README.md>)
