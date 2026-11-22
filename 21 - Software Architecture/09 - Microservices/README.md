# Microservices

Microservices separate independently deployed capabilities and introduce network failures, contract evolution, and operational ownership. Choose boundaries that a team can operate with clear data responsibilities.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
orders service owns order writes
billing service owns charge records
order status consumes billing outcomes via versioned messages
```

## 2. Common mistake

A database transaction inside one service does not provide an atomic transaction across multiple services.

## Practice

Describe one partial-failure scenario and design an idempotent recovery step.

## References

- [Official Rust documentation · ch07 00 managing growing projects with packages crates and modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

[← Previous: Modular monolith](<../08 - Modular monolith/README.md>) | [Back to Software Architecture](<../README.md>) | [Next: Event-driven architecture →](<../10 - Event-driven architecture/README.md>)
