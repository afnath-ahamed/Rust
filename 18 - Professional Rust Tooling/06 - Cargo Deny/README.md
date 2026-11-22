# Cargo Deny

Cargo Deny evaluates configured package policies for advisories, licenses, bans, and sources. Store agreed rules in version control and enforce them consistently in CI.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
cargo deny init
cargo deny check
```

## 2. Common mistake

A diagnostic can indicate policy disagreement rather than a security flaw. Document approved exceptions with scope and expiry.

## Practice

Add an approved-source policy, then verify it against an existing lockfile.

## References

- [embarkstudios.github.io · cargo deny](https://embarkstudios.github.io/cargo-deny/)
- [Official Rust documentation · index](https://doc.rust-lang.org/cargo/commands/index.html)

---

[← Previous: Cargo Audit](<../05 - Cargo Audit/README.md>) | [Back to Professional Rust Tooling](<../README.md>) | [Next: Miri →](<../07 - Miri/README.md>)
