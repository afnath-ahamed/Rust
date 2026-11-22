# Event-driven architecture

Event-driven systems publish facts or work requests for asynchronous consumers. Use stable event IDs, versioning, and duplicate handling. An outbox can commit an event record alongside its domain change.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
order transaction: update order + insert outbox record
publisher: publish committed records
consumer: apply effect with event ID deduplication
```

## 2. Common mistake

Delivery order can differ from business order unless the platform and partitioning scheme provide the required guarantee.

## Practice

Design tests for duplicate, delayed, and out-of-order events.

## References

- [Official Rust documentation · ch07 00 managing growing projects with packages crates and modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

[← Previous: Microservices](<../09 - Microservices/README.md>) | [Back to Software Architecture](<../README.md>) | [Next: Performance Engineering →](<../../22 - Performance Engineering/README.md>)
