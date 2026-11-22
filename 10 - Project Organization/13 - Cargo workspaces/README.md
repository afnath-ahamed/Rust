# Cargo workspaces

A Cargo workspace groups packages for coordinated commands and shared resolution. Use a virtual manifest when the root needs no package. Explicitly choose a resolver suitable for the workspace.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```toml
[workspace]
members = ["app", "domain"]
resolver = "3"
```

```shell
cargo test --workspace
```

## 2. Common mistake

A workspace groups packages without turning them into a single crate.

## Practice

Create app and domain as separate packages and wire a path dependency from app to domain.

## References

- [Official Rust documentation · ch07 00 managing growing projects with packages crates and modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

[← Previous: Features](<../12 - Features/README.md>) | [Back to Project Organization](<../README.md>) | [Next: Generics, Traits and Lifetimes →](<../../11 - Generics, Traits and Lifetimes/README.md>)
