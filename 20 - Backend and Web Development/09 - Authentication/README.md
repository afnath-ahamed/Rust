# Authentication

Authentication establishes the identity behind a request. Verify credentials or tokens using a maintained implementation and check the issuer, audience, validity, and configured algorithm where relevant.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
credential → verification → principal or rejection
principal: stable subject identifier and verified claims only
```

## 2. Common mistake

Reading or decoding claims alone does not verify a token. Avoid writing a custom credential verifier.

## Practice

Test missing, expired, and valid credentials using a local test identity provider or signed test fixtures.

## References

- [docs.rs · axum](https://docs.rs/axum/latest/axum/)

---

[← Previous: Diesel ORM](<../08 - Database integration/03 - Diesel ORM/README.md>) | [Back to Backend and Web Development](<../README.md>) | [Next: Authorization →](<../10 - Authorization/README.md>)
