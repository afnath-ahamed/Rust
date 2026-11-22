# Message passing

Message passing transfers values between independently running components. Model requests and responses using types so each side can handle closure and incomplete work explicitly.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let (send, recv) = std::sync::mpsc::channel();
    let writer = std::thread::spawn(move || send.send(String::from("ready")).unwrap());
    assert_eq!(recv.recv().unwrap(), "ready");
    writer.join().unwrap();
}
```

## 2. Common mistake

Retry policies need evidence about who owns a message after a failed send or receive.

## Practice

Create an enum with a work request and a shutdown request.

## References

- [Official Rust documentation · ch16 00 concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

---

[← Previous: Move closures](<../04 - Move closures/README.md>) | [Back to Concurrency and Parallelism](<../README.md>) | [Next: Channels →](<../06 - Channels/README.md>)
