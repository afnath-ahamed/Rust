# Cargo.lock

Cargo.lock records the resolved dependency versions for reproducible builds. Use Cargo commands to update it intentionally and use --locked when a build must honor the checked-in resolution.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
cargo generate-lockfile
cargo check --locked
```

## 2. Common mistake

A lockfile describes one resolution. Consumers of a library resolve dependencies using their own package or workspace lockfile.

## Practice

Change one dependency requirement and inspect the resolution diff before committing.

## References

- [Official Rust documentation · ch07 00 managing growing projects with packages crates and modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

[← Previous: Cargo.toml](<../09 - Cargo.toml/README.md>) | [Back to Project Organization](<../README.md>) | [Next: Dependencies →](<../11 - Dependencies/README.md>)
