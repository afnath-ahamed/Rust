# Packages

A Cargo package is a directory described by Cargo.toml. It can contain multiple binary targets and at most one library target.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
cargo new inventory
cd inventory
cargo metadata --no-deps --format-version 1
```

## 2. Common mistake

A package can expose several crates to the compiler through its targets.

## Practice

Inspect the metadata and locate the package name, edition, target kind, and source path.

## References

- [Official Rust documentation · ch07 00 managing growing projects with packages crates and modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

[← Previous: Error Handling](<../../09 - Error Handling/README.md>) | [Back to Project Organization](<../README.md>) | [Next: Crates →](<../02 - Crates/README.md>)
