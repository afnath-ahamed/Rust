# Beginner

Build small programs with clear inputs and outputs. Keep calculations and state transitions separate from the terminal so they can be tested with ordinary Rust functions.

## Lessons

1. [Calculator](<./01 - Calculator/README.md>)
2. [Guessing game](<./02 - Guessing game/README.md>)
3. [Temperature converter](<./03 - Temperature converter/README.md>)
4. [CLI task manager](<./04 - CLI task manager/README.md>)

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
Typical build sequence: cargo new → implement pure logic → add CLI input → test invalid cases → document usage.
Run each tool with at least three inputs, including a rejected input.
```

## 2. Common mistake

A working happy path is only the first milestone. Give rejected inputs defined behavior.

## Practice

Finish the four beginner projects with a short README, reproducible commands, and tests for their most important boundary cases.

## References

- [Official Rust documentation · ch02 00 guessing game tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

---

[← Previous: Specialization Paths](<../../25 - Specialization Paths/README.md>) | [Back to Practical Projects](<../README.md>) | [Next: Calculator →](<./01 - Calculator/README.md>)
