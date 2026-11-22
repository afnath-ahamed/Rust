# Domain-driven design

Domain-driven design focuses on the language used by the business and models invariants within bounded contexts. Aggregates define consistency boundaries for related state changes.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
Ordering context: order may be placed, paid, or cancelled.
Shipping context: shipment may be pending, dispatched, or delivered.
Translate an OrderPaid event into a shipment request at the boundary.
```

## 2. Common mistake

One overloaded model for every team can blur business terms with different meanings.

## Practice

List invariants for a small order and assign each to a value object, aggregate, or workflow.

## References

- [Official Rust documentation · ch07 00 managing growing projects with packages crates and modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

[← Previous: Hexagonal architecture](<../06 - Hexagonal architecture/README.md>) | [Back to Software Architecture](<../README.md>) | [Next: Modular monolith →](<../08 - Modular monolith/README.md>)
