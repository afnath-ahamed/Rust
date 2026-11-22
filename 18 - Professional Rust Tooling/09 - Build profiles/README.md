# Build profiles

Cargo profiles control optimization, debugging info, overflow checks, panic strategy, and other build settings. Profile settings apply from the workspace root.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```toml
[profile.release]
debug = "line-tables-only"
```

```shell
cargo build --release
```

## 2. Common mistake

Profile choices can change performance and diagnostics. Compare artifacts built under the same profile.

## Practice

Measure release artifact size with and without retained line tables.

## References

- [Official Rust documentation · index](https://doc.rust-lang.org/cargo/commands/index.html)

---

[← Previous: Compiler diagnostics](<../08 - Compiler diagnostics/README.md>) | [Back to Professional Rust Tooling](<../README.md>) | [Next: Cross-compilation →](<../10 - Cross-compilation/README.md>)
