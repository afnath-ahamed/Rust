# Clippy

Clippy adds lints for idioms, mistakes, and performance patterns. Review each suggestion against intended behavior and keep suppression local with a reason.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
rustup component add clippy
cargo clippy --workspace --all-targets -- -D warnings
```

## 2. Common mistake

all-features is useful only where the feature combinations are valid.

## Practice

Resolve a Clippy warning and show a test that verifies behavior before and after.

## References

- [Official Rust documentation · index](https://doc.rust-lang.org/cargo/commands/index.html)

---

[← Previous: Rustfmt](<../02 - Rustfmt/README.md>) | [Back to Professional Rust Tooling](<../README.md>) | [Next: Rustdoc →](<../04 - Rustdoc/README.md>)
