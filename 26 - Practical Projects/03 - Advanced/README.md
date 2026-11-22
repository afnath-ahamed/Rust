# Advanced

Build systems with concurrent clients, long-lived tasks, or cross-process work. Define capacity, failure behavior, and a reproducible local run before tuning.

## Lessons

1. [Concurrent web server](<./01 - Concurrent web server/README.md>)
2. [Real-time chat application](<./02 - Real-time chat application/README.md>)
3. [Tauri desktop application](<./03 - Tauri desktop application/README.md>)
4. [Distributed task queue](<./04 - Distributed task queue/README.md>)

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
Specify the contract → implement the simplest end-to-end path → introduce bounded concurrency → inject failure → benchmark and document.
```

## 2. Common mistake

Concurrent correctness includes outcomes when execution order, timing, or duplicate delivery differs from the happy path.

## Practice

Provide a build or reproducible local environment, plus evidence for each failure case and performance claim.

## References

- [Official Rust documentation · ch02 00 guessing game tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

---

[← Previous: REST API with database](<../02 - Intermediate/04 - REST API with database/README.md>) | [Back to Practical Projects](<../README.md>) | [Next: Concurrent web server →](<./01 - Concurrent web server/README.md>)
