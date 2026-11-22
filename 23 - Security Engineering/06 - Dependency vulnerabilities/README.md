# Dependency vulnerabilities

Dependency advisories describe known issues against specific versions. Identify direct and transitive paths, determine whether the deployed configuration is affected, and record the remediation decision.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
cargo audit
cargo tree -i dependency-name
```

## 2. Common mistake

An advisory scanner does not examine every vulnerability in application code or prove a dependency is safe.

## Practice

Take a sample advisory and write an upgrade, verification, and ownership plan.

## References

- [rustsec.org](https://rustsec.org/)

---

[← Previous: Secrets management](<../05 - Secrets management/README.md>) | [Back to Security Engineering](<../README.md>) | [Next: Supply chain security →](<../07 - Supply chain security/README.md>)
