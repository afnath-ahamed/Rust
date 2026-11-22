# SQLx

SQLx provides async database access and parameter binding. Its optional query macros can check SQL against a schema available during preparation or checking.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
cargo add sqlx --features runtime-tokio,postgres
```

```rust no_run
async fn title(pool: &sqlx::PgPool, id: i64) -> Result<Option<String>, sqlx::Error> {
    sqlx::query_scalar("SELECT title FROM notes WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}
```

## 2. Common mistake

Use bind for values. Building SQL from raw user strings loses the safety of binding.

## Practice

Apply a migration to a temporary database and test a found row, a missing row, and malformed application input.

## References

- [docs.rs · sqlx](https://docs.rs/sqlx/latest/sqlx/)
- [docs.rs · axum](https://docs.rs/axum/latest/axum/)

---

[← Previous: PostgreSQL](<../01 - PostgreSQL/README.md>) | [Back to Database integration](<../README.md>) | [Next: Diesel ORM →](<../03 - Diesel ORM/README.md>)
