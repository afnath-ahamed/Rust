# Multithreaded file processor

Build a multithreaded processor that reads many files from a test directory and performs a defined transformation. Bound parallelism and collect per-file outcomes before reporting results.

## Contents

1. [Worked example](#1-worked-example)
2. [Design check](#2-design-check)

## 1. Worked example

```text
enumerator → bounded work queue → fixed worker pool → result collector
start with line counts and compare against one-thread results
```

## 2. Design check

When result order is observable, sort by a stable key. Record unreadable files without silently dropping work and avoid unbounded output buffers.

## Acceptance criteria

- Sequential and parallel runs agree on results.
- A bad file does not block reporting for other files.
- Tests cover an empty directory, one worker, and a large queue.

## References

- [Official Rust documentation · ch02 00 guessing game tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

---

[← Previous: Expense tracker](<../02 - Expense tracker/README.md>) | [Back to Intermediate](<../README.md>) | [Next: REST API with database →](<../04 - REST API with database/README.md>)
