# Middleware

Middleware wraps request handling to apply cross-cutting behavior. Ordering determines which layers see rejections and how timeouts, metrics, request IDs, and authentication interact.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
Request ID → request logging → timeout → authentication → handler
Rejection response returns through the layers that were already entered.
```

## 2. Common mistake

A global timeout does not roll back work already committed by the handler.

## Practice

Trace a rejected request through a middleware stack and list which fields are logged.

## References

- [docs.rs · axum](https://docs.rs/axum/latest/axum/)

---

[← Previous: Routing](<../05 - Routing/README.md>) | [Back to Backend and Web Development](<../README.md>) | [Next: Serialization (Serde) →](<../07 - Serialization (Serde)/README.md>)
