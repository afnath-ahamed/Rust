# Async runtimes

An async runtime typically provides an executor, I/O integration, timers, and task management. Pick one runtime at the application boundary and enable the capabilities needed by the application.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
cargo tree -e features
```

## 2. Common mistake

A library can expose futures without starting a separate runtime on the caller's behalf.

## Practice

List the runtime capabilities used by a service that uses timers, sockets, and a blocking parser.

## References

- [Official Rust documentation · ch17 00 async await](https://doc.rust-lang.org/book/ch17-00-async-await.html)

---

[← Previous: Executors](<../05 - Executors/README.md>) | [Back to Asynchronous Programming](<../README.md>) | [Next: Tokio →](<../07 - Tokio/README.md>)
