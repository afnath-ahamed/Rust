# Secrets management

Keep secrets out of source control, tests, process arguments, and logs. Read them through an appropriate runtime configuration or secret-manager boundary and plan rotation and revocation.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
deployment identity → fetch secret with minimal access → use in restricted component
rotate: introduce replacement → verify consumers → revoke old credential
```

## 2. Common mistake

Restrict who can read diagnostics and ensure redaction applies on failure paths too.

## Practice

Sketch rotation steps for a database credential and identify how to detect successful adoption.

## References

- [rustsec.org](https://rustsec.org/)

---

[← Previous: Cryptography fundamentals](<../04 - Cryptography fundamentals/README.md>) | [Back to Security Engineering](<../README.md>) | [Next: Dependency vulnerabilities →](<../06 - Dependency vulnerabilities/README.md>)
