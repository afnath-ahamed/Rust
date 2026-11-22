# Cryptography fundamentals

Cryptographic primitives address distinct needs. Hashing summarizes data, MACs authenticate with a shared secret, and authenticated encryption protects confidentiality and detects tampering. Use reviewed libraries and protocols for real applications.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
Password storage → purpose-built password hashing
Message authenticity with a shared key → MAC
Encrypted stored data with integrity → authenticated encryption and key management
```

## 2. Common mistake

Cryptographic algorithms and formats require safe key and nonce management. Avoid custom constructions.

## Practice

For a signed download, describe what a trusted signature proves and which identity must be trusted.

## References

- [rustsec.org](https://rustsec.org/)

---

[← Previous: Secure coding practices](<../03 - Secure coding practices/README.md>) | [Back to Security Engineering](<../README.md>) | [Next: Secrets management →](<../05 - Secrets management/README.md>)
