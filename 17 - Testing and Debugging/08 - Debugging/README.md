# Debugging

Start debugging with a minimal reproduction. Reduce the input while preserving the failure, then use diagnostics, a debugger, or a backtrace to confirm the cause.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```powershell
$env:RUST_BACKTRACE = '1'
cargo test failing_case -- --nocapture
```

## 2. Common mistake

A backtrace shows the path to failure. The root cause can be an earlier state mutation.

## Practice

Reduce a failing example to the fewest lines and keep the case as a test.

## References

- [Official Rust documentation · ch11 00 testing](https://doc.rust-lang.org/book/ch11-00-testing.html)

---

[← Previous: Benchmarking](<../07 - Benchmarking/README.md>) | [Back to Testing and Debugging](<../README.md>) | [Next: Logging →](<../09 - Logging/README.md>)
