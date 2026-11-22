# Publishing crates

Publishing a crate creates a versioned artifact for users. Review package contents, licensing, documentation, and the intended registry before release. Start with the dry run.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
cargo package --list
cargo publish --dry-run
```

## 2. Common mistake

Keep credentials and local data out of package contents. Treat a published version as permanent.

## Practice

Prepare a local package and inspect its packaged files and generated README before considering publication.

## References

- [Official Rust documentation · index](https://doc.rust-lang.org/cargo/commands/index.html)

---

[← Previous: Cross-compilation](<../10 - Cross-compilation/README.md>) | [Back to Professional Rust Tooling](<../README.md>) | [Next: Data Structures and Algorithms →](<../../19 - Data Structures and Algorithms/README.md>)
