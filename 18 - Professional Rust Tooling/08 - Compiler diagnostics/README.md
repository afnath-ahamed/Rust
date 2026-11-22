# Compiler diagnostics

Read a diagnostic from the main error and span through notes, labels, and suggestions. Simplify generic types and fix the first causal error before chasing cascades.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
rustc --explain E0382
cargo check
```

## 2. Common mistake

A suggestion that compiles may alter ownership, allocation, or API behavior. Verify its intent.

## Practice

Trigger a moved-value error in a small experiment, then fix it using a borrow.

## References

- [Official Rust documentation · index](https://doc.rust-lang.org/cargo/commands/index.html)

---

[← Previous: Miri](<../07 - Miri/README.md>) | [Back to Professional Rust Tooling](<../README.md>) | [Next: Build profiles →](<../09 - Build profiles/README.md>)
