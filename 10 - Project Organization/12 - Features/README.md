# Features

Features allow additive build-time options. Expose optional capabilities through manifest features and guard code with cfg(feature = "..."). Test both enabled and disabled builds.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```toml
[features]
default = []
reporting = []
```

```shell
cargo test --no-default-features
cargo test --features reporting
```

## 2. Common mistake

Treat mutually exclusive modes as a design smell because downstream feature requests can be combined.

## Practice

Add a reporting function behind a feature and verify both configurations compile.

## Related lesson

[Cargo commands](<../../18 - Professional Rust Tooling/01 - Cargo commands/README.md>)

## References

- [Official Rust documentation · ch07 00 managing growing projects with packages crates and modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

[← Previous: Dependencies](<../11 - Dependencies/README.md>) | [Back to Project Organization](<../README.md>) | [Next: Cargo workspaces →](<../13 - Cargo workspaces/README.md>)
