# Rustfmt

Rustfmt standardizes source layout. Run it before sharing changes and use check mode in CI so formatting differences are reported without editing files.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
rustup component add rustfmt
cargo fmt --all -- --check
```

## 2. Common mistake

Use the project toolchain and existing configuration to keep formatting reproducible.

## Practice

Apply formatting to a small test change and confirm a second run produces no diff.

## References

- [Official Rust documentation · index](https://doc.rust-lang.org/cargo/commands/index.html)

---

[← Previous: Cargo commands](<../01 - Cargo commands/README.md>) | [Back to Professional Rust Tooling](<../README.md>) | [Next: Clippy →](<../03 - Clippy/README.md>)
