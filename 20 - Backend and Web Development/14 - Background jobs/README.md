# Background jobs

Background jobs move work off the request path. Record a stable job identity, acknowledge only at the intended completion point, and use bounded retries with a failure destination.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
queued → leased → completed
leased → expired or retryable failure → queued
leased → attempts exhausted → failed
```

## 2. Common mistake

A worker can commit an effect and crash before acknowledging it. Design the effect to tolerate retry.

## Practice

Specify a job for thumbnail generation and test duplicate delivery and retry exhaustion.

## References

- [docs.rs · axum](https://docs.rs/axum/latest/axum/)

---

[← Previous: Caching](<../13 - Caching/README.md>) | [Back to Backend and Web Development](<../README.md>) | [Next: Software Architecture →](<../../21 - Software Architecture/README.md>)
