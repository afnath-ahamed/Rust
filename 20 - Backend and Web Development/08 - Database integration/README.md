# Database integration

A database integration needs parameter binding, connection limits, migrations, and clear transaction boundaries. Keep the database adapter behind an interface where it helps separate persistence from policy.

## Lessons

1. [PostgreSQL](<./01 - PostgreSQL/README.md>)
2. [SQLx](<./02 - SQLx/README.md>)
3. [Diesel ORM](<./03 - Diesel ORM/README.md>)

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
request → validation → acquire pooled connection → transaction → commit → response
test path: apply migrations to a disposable database before testing the adapter
```

## 2. Common mistake

An in-memory fake cannot verify real database constraints, isolation, or SQL syntax.

## Practice

Write a persistence integration test that proves an unsuccessful update leaves the data unchanged.

## References

- [docs.rs · axum](https://docs.rs/axum/latest/axum/)

---

[← Previous: Serialization (Serde)](<../07 - Serialization (Serde)/README.md>) | [Back to Backend and Web Development](<../README.md>) | [Next: PostgreSQL →](<./01 - PostgreSQL/README.md>)
