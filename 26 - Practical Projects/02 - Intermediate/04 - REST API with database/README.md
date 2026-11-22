# REST API with database

Build a notes API backed by a disposable PostgreSQL instance. Implement migrations and parameterized access, then exercise the HTTP contract through real persistence tests.

## Contents

1. [Worked example](#1-worked-example)
2. [Design check](#2-design-check)

## 1. Worked example

```text
POST /notes → validate and insert → 201 + Location
GET /notes/{id} → select bound identifier → 200 or 404
DELETE /notes/{id} → authorize, delete, and return a documented status
```

## 2. Design check

Make persistence error mapping explicit. Validate client input before starting database work and use migration ordering that supports a fresh test database.

## Acceptance criteria

- A fresh database migrates automatically in the test setup.
- Tests distinguish invalid JSON, invalid values, missing IDs, and database failure.
- Data persists across a service restart.

## References

- [Official Rust documentation · ch02 00 guessing game tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

---

[← Previous: Multithreaded file processor](<../03 - Multithreaded file processor/README.md>) | [Back to Intermediate](<../README.md>) | [Next: Advanced →](<../../03 - Advanced/README.md>)
