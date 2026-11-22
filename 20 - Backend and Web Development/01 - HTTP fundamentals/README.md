# HTTP fundamentals

An HTTP request carries a method, target, headers, and possibly a body. A response carries a status, headers, and possibly a representation. Methods express semantics such as safety and idempotence.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```http
GET /notes/7 HTTP/1.1
Host: localhost:3000
Accept: application/json

HTTP/1.1 200 OK
Content-Type: application/json

{"id": 7, "title": "Rust"}
```

## 2. Common mistake

Transport success and a 2xx application response are distinct. Test status and body together.

## Practice

Write contracts for a successful GET, a missing record, and a JSON request with an unsupported content type.

## References

- [docs.rs · axum](https://docs.rs/axum/latest/axum/)

---

[← Previous: Data Structures and Algorithms](<../../19 - Data Structures and Algorithms/README.md>) | [Back to Backend and Web Development](<../README.md>) | [Next: REST APIs →](<../02 - REST APIs/README.md>)
