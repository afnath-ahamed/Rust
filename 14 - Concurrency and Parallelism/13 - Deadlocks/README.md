# Deadlocks

Deadlock occurs when tasks wait indefinitely for each other. A consistent global acquisition order rules out simple circular waits involving those locks.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
Path A: lock account 1, then lock account 2
Path B: lock account 1, then lock account 2
Both paths release both guards before performing I/O.
```

## 2. Common mistake

A global order helps only when all code paths follow it, including callbacks.

## Practice

Find a conflicting order in a two-resource transfer design and repair both paths.

## References

- [Official Rust documentation · ch16 00 concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

---

[← Previous: Race conditions](<../12 - Race conditions/README.md>) | [Back to Concurrency and Parallelism](<../README.md>) | [Next: Rayon →](<../14 - Rayon/README.md>)
