# Secure coding practices

Secure coding begins with explicit trust boundaries and least privilege. Use parameterized database operations, checked filesystem access, safe defaults, and error messages appropriate to the boundary.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
public response: request could not be completed + request ID
restricted logs: operation and structured error category
redacted everywhere: credentials and raw sensitive payloads
```

## 2. Common mistake

A request that parses successfully may still be unauthorized or too expensive to execute.

## Practice

Write a review checklist for one handler covering identity, validation, data access, and response redaction.

## References

- [rustsec.org](https://rustsec.org/)

---

[← Previous: Input validation](<../02 - Input validation/README.md>) | [Back to Security Engineering](<../README.md>) | [Next: Cryptography fundamentals →](<../04 - Cryptography fundamentals/README.md>)
