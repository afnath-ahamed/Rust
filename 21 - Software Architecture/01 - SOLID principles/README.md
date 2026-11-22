# SOLID principles

SOLID is a set of design heuristics about responsibility, extension, substitution, interface size, and dependency direction. Apply them to concrete sources of change in Rust modules and traits.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
Import parsing changes independently of persistence rules.
Keep parser and repository boundaries separate, then compose them in one use case.
```

## 2. Common mistake

Splitting every function behind a trait adds ceremony without automatically improving changeability.

## Practice

Find one module with two reasons to change and propose a smaller public API.

## References

- [Official Rust documentation · ch07 00 managing growing projects with packages crates and modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

[← Previous: Backend and Web Development](<../../20 - Backend and Web Development/README.md>) | [Back to Software Architecture](<../README.md>) | [Next: Design patterns →](<../02 - Design patterns/README.md>)
