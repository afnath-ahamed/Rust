# Hexagonal architecture

Hexagonal architecture describes application ports for interactions and adapters that translate specific technologies to those ports. An inbound port invokes a use case. An outbound port enables a use case to request an external capability.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
CLI adapter → CreateNote port → application → NoteStore port ← PostgreSQL adapter
```

## 2. Common mistake

A port named after a framework often exposes implementation concerns at the wrong level.

## Practice

Implement an in-memory adapter for one outbound port and run the same contract tests against both adapters.

## References

- [Official Rust documentation · ch07 00 managing growing projects with packages crates and modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

[← Previous: Clean architecture](<../05 - Clean architecture/README.md>) | [Back to Software Architecture](<../README.md>) | [Next: Domain-driven design →](<../07 - Domain-driven design/README.md>)
