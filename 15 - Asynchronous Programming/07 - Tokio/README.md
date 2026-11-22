# Tokio

Tokio is a Rust async runtime. Its main attribute is a convenient entry point when the required features are enabled. An application can also construct a runtime explicitly.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
cargo add tokio --features macros,rt-multi-thread,time
```

```rust no_run
#[tokio::main]
async fn main() {
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    println!("ready");
}
```

## 2. Common mistake

Match dependency features to the APIs you actually use.

## Practice

Use two timers and compare sequential waiting with tokio::join!.

## References

- [Official Rust documentation · ch17 00 async await](https://doc.rust-lang.org/book/ch17-00-async-await.html)

---

[← Previous: Async runtimes](<../06 - Async runtimes/README.md>) | [Back to Asynchronous Programming](<../README.md>) | [Next: Async tasks →](<../08 - Async tasks/README.md>)
