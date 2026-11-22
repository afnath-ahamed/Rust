# Payment processing system

Build a payment processing simulator with exact minor units, idempotent submission, a test provider, and an append-only audit record. Model transitions and reconciliation before adding any external integration.

## Contents

1. [Worked example](#1-worked-example)
2. [Design check](#2-design-check)

## 1. Worked example

```text
request with idempotency key → validate amount and currency → persist pending operation
test provider → approved/declined/unknown → record outcome and reconcile
replay same key → return recorded outcome without a duplicate effect
```

## 2. Design check

Use simulated funds and provider fixtures. A timed-out request can have an unknown outcome and must be reconciled before unsafe replay.

## Acceptance criteria

- Duplicate requests produce one simulated effect.
- Tests cover overflow, currency mismatch, declined payment, and timeout with later success.
- A reconciliation report explains mismatches and preserves history.

## References

- [Official Rust documentation · ch02 00 guessing game tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

---

[← Previous: Production-grade backend](<../01 - Production-grade backend/README.md>) | [Back to Professional](<../README.md>) | [Next: High-performance caching server →](<../03 - High-performance caching server/README.md>)
