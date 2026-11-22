# Layered architecture

Layered architecture groups responsibilities such as transport, use cases, and persistence. Boundaries define where protocol conversion, orchestration, and persistence happen.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
HTTP handler → application use case → repository → database
HTTP converts wire input into a command, and the repository translates stored rows.
```

## 2. Common mistake

Leaking database row shapes out of the persistence layer couples storage and public API changes.

## Practice

Trace one validation error and decide where it becomes an HTTP status.

## References

- [Official Rust documentation · ch07 00 managing growing projects with packages crates and modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

[← Previous: Dependency injection](<../03 - Dependency injection/README.md>) | [Back to Software Architecture](<../README.md>) | [Next: Clean architecture →](<../05 - Clean architecture/README.md>)
