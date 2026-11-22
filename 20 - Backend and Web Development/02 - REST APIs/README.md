# REST APIs

A resource-oriented API models entities with stable identifiers and operations using HTTP semantics. Define validation, status codes, pagination, and failure response shape as part of the contract.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
POST /notes     -> 201 with Location and new representation
GET /notes/7    -> 200 with representation or 404
DELETE /notes/7 -> 204 with an empty body when removal succeeds
```

## 2. Common mistake

A repeated POST can create duplicates unless the service defines deduplication or an idempotency contract.

## Practice

Specify a cursor-based list and test whether new inserts create duplicates between pages.

## References

- [docs.rs · axum](https://docs.rs/axum/latest/axum/)

---

[← Previous: HTTP fundamentals](<../01 - HTTP fundamentals/README.md>) | [Back to Backend and Web Development](<../README.md>) | [Next: Axum →](<../03 - Axum/README.md>)
