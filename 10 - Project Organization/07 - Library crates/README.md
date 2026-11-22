# Library crates

A library defines reusable APIs for other crates. Keep business logic callable and testable in src/lib.rs, with command-line or service startup code in separate binary targets.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
cargo new ratings --lib
src/lib.rs exports a pure score function
tests/scoring.rs checks the public function as a downstream caller
```

## 2. Common mistake

A library has no program entry point of its own.

## Practice

Create a library that exposes a validated calculation and exercise only its public API from tests.

## References

- [Official Rust documentation · ch07 00 managing growing projects with packages crates and modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

[← Previous: Use keyword](<../06 - Use keyword/README.md>) | [Back to Project Organization](<../README.md>) | [Next: Binary crates →](<../08 - Binary crates/README.md>)
