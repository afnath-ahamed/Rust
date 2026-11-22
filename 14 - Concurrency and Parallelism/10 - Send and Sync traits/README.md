# Send and Sync traits

Send means ownership can cross threads. Sync means shared references can cross threads because &T is Send. The compiler generally derives these properties from a type's fields.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<std::sync::Arc<String>>();
}
```

## 2. Common mistake

Wrapping a type in Arc does not repair a non-Send or non-Sync payload.

## Practice

Try a compile-time assertion for Rc<String> and identify the difference.

## Related lesson

[Traits](<../../11 - Generics, Traits and Lifetimes/04 - Traits/README.md>)

## References

- [Official Rust documentation · ch16 00 concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

---

[← Previous: RwLock](<../09 - RwLock/README.md>) | [Back to Concurrency and Parallelism](<../README.md>) | [Next: Atomics →](<../11 - Atomics/README.md>)
