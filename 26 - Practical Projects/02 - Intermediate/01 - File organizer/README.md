# File organizer

Build a file organizer that classifies files by a documented rule and creates a move plan first. Apply changes only when explicitly selected. Define how name collisions and interrupted operations are reported.

## Contents

1. [Worked example](#1-worked-example)
2. [Design check](#2-design-check)

## 1. Worked example

```text
input: disposable fixture directory and category map
plan: source path → proposed destination, without editing files
apply: verify containment, handle collisions, move, and log each outcome
```

## 2. Design check

Do not traverse symbolic links by default. Recheck source and destination assumptions before touching files and avoid overwriting an existing file by default.

## Acceptance criteria

- Dry run and apply agree on a stable fixture set.
- Tests cover collisions, unknown extensions, nested paths, and partial failures.
- An actionable report lists completed and skipped operations separately.

## References

- [Official Rust documentation · ch02 00 guessing game tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

---

[Back to Intermediate](<../README.md>) | [Next: Expense tracker →](<../02 - Expense tracker/README.md>)
