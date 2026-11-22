# Cargo.toml

Cargo.toml is the source manifest for package metadata, targets, dependencies, and features. Keep its declarations readable and document platform-specific choices.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```toml
[package]
name = "inventory"
version = "0.1.0"
edition = "2024"
```

```shell
cargo check
```

## 2. Common mistake

The edition specifies language compatibility and is distinct from the minimum supported compiler documented by rust-version.

## Practice

Add a license field and inspect the manifest via Cargo metadata.

## References

- [Official Rust documentation · ch07 00 managing growing projects with packages crates and modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

[← Previous: Binary crates](<../08 - Binary crates/README.md>) | [Back to Project Organization](<../README.md>) | [Next: Cargo.lock →](<../10 - Cargo.lock/README.md>)
