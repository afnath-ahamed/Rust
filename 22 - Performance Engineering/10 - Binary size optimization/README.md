# Binary size optimization

Binary size depends on code generation, included features, dependencies, symbol information, and target linking. Use size optimization and feature pruning when distribution constraints justify the trade-offs.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```toml
[profile.release]
opt-level = "s"
strip = "symbols"
```

## 2. Common mistake

Keep deployable debug symbols or symbol mappings when operations need postmortem diagnostics.

## Practice

Measure size before and after reducing a dependency feature and check startup behavior.

## References

- [Official Rust documentation · profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)

---

[← Previous: Release optimization](<../09 - Release optimization/README.md>) | [Back to Performance Engineering](<../README.md>) | [Next: Security Engineering →](<../../23 - Security Engineering/README.md>)
