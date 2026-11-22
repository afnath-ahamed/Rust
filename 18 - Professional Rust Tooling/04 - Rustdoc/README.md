# Rustdoc

Rustdoc renders rust API docs from doc comments and examples. Generated docs should make behavior, return values, and failure or safety conditions easy for callers to find.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
cargo doc --no-deps --open
cargo test --doc
```

## 2. Common mistake

An example shown in the documentation is part of the learning experience. Keep it runnable when practical.

## Practice

Document a public API with an Errors section and a doctest.

## References

- [Official Rust documentation · index](https://doc.rust-lang.org/cargo/commands/index.html)

---

[← Previous: Clippy](<../03 - Clippy/README.md>) | [Back to Professional Rust Tooling](<../README.md>) | [Next: Cargo Audit →](<../05 - Cargo Audit/README.md>)
