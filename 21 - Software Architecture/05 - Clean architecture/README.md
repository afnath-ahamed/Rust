# Clean architecture

Clean architecture arranges dependencies so policy stays independent of framework details. Use-case code can own interfaces implemented by outer adapters.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
HTTP adapter → application policy ← storage adapter
The binary assembles policy and adapters. The use case accepts a domain command.
```

## 2. Common mistake

A trait alone does not establish a boundary if its parameters expose driver or framework-specific types.

## Practice

Move one framework type from the use-case API to an adapter and write a unit test.

## References

- [Official Rust documentation · ch07 00 managing growing projects with packages crates and modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

[← Previous: Layered architecture](<../04 - Layered architecture/README.md>) | [Back to Software Architecture](<../README.md>) | [Next: Hexagonal architecture →](<../06 - Hexagonal architecture/README.md>)
