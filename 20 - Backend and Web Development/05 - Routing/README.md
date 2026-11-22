# Routing

A router maps methods and paths to handlers. Path parameters identify resources, queries select filters, and request bodies carry input representations.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
GET /notes/{id}             read one note
GET /notes?limit=10         list notes
PUT /notes/{id}             replace one note
```

## 2. Common mistake

Keep route, method, and input parsing separate from domain authorization.

## Practice

Design routes for a nested comments collection and decide which identifiers each handler needs.

## References

- [docs.rs · axum](https://docs.rs/axum/latest/axum/)

---

[← Previous: Actix Web](<../04 - Actix Web/README.md>) | [Back to Backend and Web Development](<../README.md>) | [Next: Middleware →](<../06 - Middleware/README.md>)
