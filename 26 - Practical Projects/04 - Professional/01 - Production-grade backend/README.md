# Production-grade backend

Build a production-style backend with validated APIs, persistence, authorization, and telemetry. Keep a single deployable until independent deployability is a measured need. Include operational documentation.

## Contents

1. [Worked example](#1-worked-example)
2. [Design check](#2-design-check)

## 1. Worked example

```text
CI: format, lint, tests, dependency review
release: build locked artifact → deploy to staging → run smoke tests
operations: health endpoints, trace correlation, documented rollback
```

## 2. Design check

Plan failure behavior for migrations, downtime, and traffic spikes. Test rollback compatibility with persisted data.

## Acceptance criteria

- Reproducible local startup, database migrations, contract tests, and scoped authorization pass.
- An outage and recovery drill are documented with signals and timelines.
- A release artifact can be rolled back according to a tested runbook.

## References

- [Official Rust documentation · ch02 00 guessing game tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

---

[Back to Professional](<../README.md>) | [Next: Payment processing system →](<../02 - Payment processing system/README.md>)
