# Diesel ORM

Diesel provides a typed query builder with schema-aware expression types. Use migrations for schema changes and query a small repository adapter with testable public behavior.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
1. Define the notes table with a migration.
2. Generate or update the schema module.
3. Filter notes by identifier using the typed query builder.
4. Test row decoding against a PostgreSQL database.
```

## 2. Common mistake

Diesel's standard connection is blocking. Schedule it appropriately in async servers or use an async integration suitable for the project.

## Practice

Retrieve the same note with SQLx and Diesel and compare what each compiler validates.

## References

- [diesel.rs · getting started](https://diesel.rs/guides/getting-started.html)
- [docs.rs · axum](https://docs.rs/axum/latest/axum/)

---

[← Previous: SQLx](<../02 - SQLx/README.md>) | [Back to Database integration](<../README.md>) | [Next: Authentication →](<../../09 - Authentication/README.md>)
