# Intermediate

Connect multiple Rust concepts across files, typed data, or background work. Start from a public interface, then add persistence and representative failure tests.

## Lessons

1. [File organizer](<./01 - File organizer/README.md>)
2. [Expense tracker](<./02 - Expense tracker/README.md>)
3. [Multithreaded file processor](<./03 - Multithreaded file processor/README.md>)
4. [REST API with database](<./04 - REST API with database/README.md>)

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
Use a domain module, an I/O adapter, and an entry point. Start with a local test environment and document exact setup commands.
```

## 2. Common mistake

Keep test fixtures local and reproducible. Real account credentials and production data are unnecessary for these builds.

## Practice

Submit a runnable project for each challenge with input fixtures, failure tests, and observable outcomes.

## References

- [Official Rust documentation · ch02 00 guessing game tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

---

[← Previous: CLI task manager](<../01 - Beginner/04 - CLI task manager/README.md>) | [Back to Practical Projects](<../README.md>) | [Next: File organizer →](<./01 - File organizer/README.md>)
