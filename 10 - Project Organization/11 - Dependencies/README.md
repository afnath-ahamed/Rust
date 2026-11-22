# Dependencies

Dependency declarations can come from a registry, a path, or Git. Normal, dev, and build dependencies serve different compilation stages. Restrict features to what the package uses.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
cargo add serde --features derive
cargo add --dev proptest
cargo tree -e features
```

## 2. Common mistake

Cargo features are additive across the resolved usages where feature unification applies. Check the resolved feature graph instead of assuming each request is isolated.

## Practice

Compare the dependency tree before and after enabling an optional feature.

## References

- [Official Rust documentation · ch07 00 managing growing projects with packages crates and modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

[← Previous: Cargo.lock](<../10 - Cargo.lock/README.md>) | [Back to Project Organization](<../README.md>) | [Next: Features →](<../12 - Features/README.md>)
