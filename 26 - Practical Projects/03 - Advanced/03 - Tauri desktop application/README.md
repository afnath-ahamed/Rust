# Tauri desktop application

Build a small desktop notes app with Tauri. Keep the Rust commands narrow and use frontend state for presentation. Validate the command boundary and store data in a defined app location.

## Contents

1. [Worked example](#1-worked-example)
2. [Design check](#2-design-check)

## 1. Worked example

```text
commands: list_notes, save_note, delete_note
storage: app-scoped directory with test fixtures in a separate temp directory
workflow: create, edit, search, close, reopen
```

## 2. Design check

Scope frontend capabilities to the required commands and data. Treat frontend payloads as untrusted and validate storage paths.

## Acceptance criteria

- Notes survive restart and the UI reports storage errors.
- Invalid payloads and paths outside allowed storage are rejected.
- Document the build and platform prerequisites.

## References

- [Official Rust documentation · ch02 00 guessing game tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

---

[← Previous: Real-time chat application](<../02 - Real-time chat application/README.md>) | [Back to Advanced](<../README.md>) | [Next: Distributed task queue →](<../04 - Distributed task queue/README.md>)
