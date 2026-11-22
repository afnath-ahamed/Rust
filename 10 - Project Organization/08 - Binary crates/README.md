# Binary crates

A binary target builds an executable. src/main.rs is the conventional default. src/bin can hold separate entry points for the same package.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
cargo run --bin worker -- --help
```

## 2. Common mistake

Arguments after the second separator are passed to the application. Cargo still needs the named binary target to exist.

## Practice

Add src/bin/worker.rs beside src/main.rs and run each explicitly.

## References

- [Official Rust documentation · ch07 00 managing growing projects with packages crates and modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

[← Previous: Library crates](<../07 - Library crates/README.md>) | [Back to Project Organization](<../README.md>) | [Next: Cargo.toml →](<../09 - Cargo.toml/README.md>)
