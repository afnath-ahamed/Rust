# Cross-compilation

Cross-compiling requires a target triple plus compatible linkers, native libraries, and tooling when the target needs them. A successful cross-build still needs testing on the target or a supported runner.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
rustc --print target-list
rustup target add aarch64-unknown-linux-gnu
cargo check --target aarch64-unknown-linux-gnu
```

## 2. Common mistake

Adding a target installs Rust target support. It does not necessarily install the target system linker or native SDK.

## Practice

List the target and host dependencies for a small package with one native dependency.

## References

- [Official Rust documentation · index](https://doc.rust-lang.org/cargo/commands/index.html)

---

[← Previous: Build profiles](<../09 - Build profiles/README.md>) | [Back to Professional Rust Tooling](<../README.md>) | [Next: Publishing crates →](<../11 - Publishing crates/README.md>)
