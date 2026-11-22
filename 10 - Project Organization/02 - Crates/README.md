# Crates

A crate is a compilation unit. Its root is usually src/main.rs for a default binary or src/lib.rs for a default library. Modules form the hierarchy inside that crate.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
inventory/
├── Cargo.toml
└── src/
    ├── lib.rs  # library crate root
    └── main.rs # binary crate root
```

## 2. Common mistake

A module file does not become a module merely by existing in the directory.

## Practice

Add a library target to a binary package and identify both targets in cargo metadata.

## References

- [Official Rust documentation · ch07 00 managing growing projects with packages crates and modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

[← Previous: Packages](<../01 - Packages/README.md>) | [Back to Project Organization](<../README.md>) | [Next: Modules →](<../03 - Modules/README.md>)
