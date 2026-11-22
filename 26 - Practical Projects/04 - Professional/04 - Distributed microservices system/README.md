# Distributed microservices system

Build a local multi-service workflow with separate data ownership, versioned contracts, and observable retries. Keep the first slice small enough to test delayed, duplicated, and missing messages.

## Contents

1. [Worked example](#1-worked-example)
2. [Design check](#2-design-check)

## 1. Worked example

```text
orders service writes order and outbox event
billing simulator handles event idempotently and emits outcome
orders consumes outcome and updates status
trace a request across both services with a correlation identifier
```

## 2. Design check

Network success can be lost after a peer commits. Each service needs an idempotency rule, and payload evolution needs a compatible plan.

## Acceptance criteria

- Workflow recovers from a temporary service outage.
- Duplicate and out-of-order messages have tested outcomes.
- Trace correlation and a rollback procedure are documented.

## References

- [Official Rust documentation · ch02 00 guessing game tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

---

[← Previous: High-performance caching server](<../03 - High-performance caching server/README.md>) | [Back to Professional](<../README.md>) | [Next: Open-source Rust contribution →](<../05 - Open-source Rust contribution/README.md>)
