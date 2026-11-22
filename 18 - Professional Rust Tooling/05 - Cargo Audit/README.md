# Cargo Audit

Cargo Audit checks your lockfile against the RustSec advisory database. Triage results by version, dependency reachability, and project policy. Record remediation and any temporary exception.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
cargo audit
cargo tree -i dependency-name
```

## 2. Common mistake

A clean scan covers published advisories known to the data source. Review your own code and dependency changes as separate activities.

## Practice

Inspect one transitive dependency and document a workflow for upgrading it if an advisory affects your use.

## Related lesson

[Dependency vulnerabilities](<../../23 - Security Engineering/06 - Dependency vulnerabilities/README.md>)

## References

- [rustsec.org](https://rustsec.org/)
- [Official Rust documentation · index](https://doc.rust-lang.org/cargo/commands/index.html)

---

[← Previous: Rustdoc](<../04 - Rustdoc/README.md>) | [Back to Professional Rust Tooling](<../README.md>) | [Next: Cargo Deny →](<../06 - Cargo Deny/README.md>)
