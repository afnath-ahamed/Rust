# Async / Await

async makes a future and await yields until that future is ready. Awaiting work one step at a time preserves dependencies. Running independent work concurrently is a separate decision.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    async fn fetch_count() -> u32 {
        4
    }
    async fn report() -> u32 {
        fetch_count().await * 2
    }
    let _future = report();
}
```

## 2. Common mistake

await can only be used in async contexts. Choose an executor at the program boundary.

## Practice

Run report under a runtime and assert its result is 8.

## References

- [Official Rust documentation · ch17 00 async await](https://doc.rust-lang.org/book/ch17-00-async-await.html)

---

[← Previous: Blocking vs Non-blocking](<../02 - Blocking vs Non-blocking/README.md>) | [Back to Asynchronous Programming](<../README.md>) | [Next: Futures →](<../04 - Futures/README.md>)
