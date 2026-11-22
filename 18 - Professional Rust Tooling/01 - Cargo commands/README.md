# Cargo commands

Use cargo check for type checking, build for artifacts, run for execution, and test for the test harness. Cargo commands can target a package or an entire workspace.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
cargo check
cargo run -- --help
cargo test --workspace
```

## 2. Common mistake

cargo check skips final code generation and linking, so a later build can still expose linker or build-mode issues.

## Practice

Run check and build on the same project and locate the resulting artifact.

## References

- [Official Rust documentation · index](https://doc.rust-lang.org/cargo/commands/index.html)

---

[← Previous: Testing and Debugging](<../../17 - Testing and Debugging/README.md>) | [Back to Professional Rust Tooling](<../README.md>) | [Next: Rustfmt →](<../02 - Rustfmt/README.md>)
