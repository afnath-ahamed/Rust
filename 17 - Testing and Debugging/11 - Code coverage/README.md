# Code coverage

Coverage reports which lines or branches ran under tests. Treat uncovered meaningful paths as a prompt to design cases. Executed code can still have no meaningful assertions.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
cargo llvm-cov --html
```

## 2. Common mistake

Install the coverage tool and required LLVM components according to its documentation. Generated reports may contain paths and source material.

## Practice

Add a missing failure-path test and compare the coverage report before and after.

## References

- [Official Rust documentation · ch11 00 testing](https://doc.rust-lang.org/book/ch11-00-testing.html)

---

[← Previous: Tracing](<../10 - Tracing/README.md>) | [Back to Testing and Debugging](<../README.md>) | [Next: Professional Rust Tooling →](<../../18 - Professional Rust Tooling/README.md>)
