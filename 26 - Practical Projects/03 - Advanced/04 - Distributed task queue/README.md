# Distributed task queue

Build a distributed queue on a local datastore with submission, leases, acknowledgments, and retry limits. Use stable job IDs and design worker effects to tolerate at-least-once processing.

## Contents

1. [Worked example](#1-worked-example)
2. [Design check](#2-design-check)

## 1. Worked example

```text
enqueue → lease(worker, deadline) → effect → acknowledge
worker crash → lease expires → eligible retry
attempt limit reached → dead-letter state
```

## 2. Design check

A worker may finish its effect but lose the lease before acknowledging. Document the deduplication strategy and use a fake clock in tests.

## Acceptance criteria

- A crashed worker's job is retried and duplicate delivery is safe.
- Exhausted jobs become inspectable without blocking others.
- A concurrency test verifies that active leases are assigned atomically.

## References

- [Official Rust documentation · ch02 00 guessing game tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

---

[← Previous: Tauri desktop application](<../03 - Tauri desktop application/README.md>) | [Back to Advanced](<../README.md>) | [Next: Professional →](<../../04 - Professional/README.md>)
