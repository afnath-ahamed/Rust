# PostgreSQL

PostgreSQL provides relational tables, constraints, indexes, and transactions. Define important invariants in the schema so concurrent clients cannot bypass application-only checks.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```sql
CREATE TABLE notes (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title text NOT NULL CHECK (length(trim(title)) > 0)
);
```

## 2. Common mistake

Adding an index improves selected reads while increasing write and storage cost.

## Practice

Use a disposable database to test the constraint, then inspect an indexed lookup with EXPLAIN.

## References

- [docs.rs · axum](https://docs.rs/axum/latest/axum/)

---

[← Previous: Database integration](<../README.md>) | [Back to Database integration](<../README.md>) | [Next: SQLx →](<../02 - SQLx/README.md>)
