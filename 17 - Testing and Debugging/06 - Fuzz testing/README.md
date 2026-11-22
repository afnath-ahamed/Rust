# Fuzz testing

Fuzz testing repeatedly feeds varied input to your own parser or library and checks robustness, such as absence of panics and compliance with resource limits. Keep minimized failing inputs as regression cases.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
Target: a local record parser with a bounded byte slice input
Seeds: empty record, one valid record, one truncated field
Invariant: parsing reports a value or structured error for every permitted input size
```

## 2. Common mistake

An unconstrained target can spend its time allocating huge buffers rather than exercising parser logic.

## Practice

Write a local parser property for malformed UTF-8 and preserve a reduced failure case.

## References

- [rust-fuzz.github.io · book](https://rust-fuzz.github.io/book/)
- [Official Rust documentation · ch11 00 testing](https://doc.rust-lang.org/book/ch11-00-testing.html)

---

[← Previous: Mocking](<../05 - Mocking/README.md>) | [Back to Testing and Debugging](<../README.md>) | [Next: Benchmarking →](<../07 - Benchmarking/README.md>)
