# Benchmarking

Benchmarking measures repeatable performance under controlled input and build settings. Establish a baseline and collect multiple runs for representative input sizes.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
cargo bench
```

## 2. Common mistake

A debug build or optimized-away workload can distort the result. Consume the measured output and verify correctness outside timing.

## Practice

Compare one operation at three input sizes and record the benchmark environment.

## References

- [Official Rust documentation · ch11 00 testing](https://doc.rust-lang.org/book/ch11-00-testing.html)

---

[← Previous: Fuzz testing](<../06 - Fuzz testing/README.md>) | [Back to Testing and Debugging](<../README.md>) | [Next: Debugging →](<../08 - Debugging/README.md>)
