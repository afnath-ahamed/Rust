# `Arc<T>`

Arc<T> uses atomic reference counts to support shared ownership across threads when T has the required thread-safety properties. Synchronize mutation separately.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    use std::sync::Arc;
    let items = Arc::new(vec![1, 2, 3]);
    let worker_items = Arc::clone(&items);
    let worker = std::thread::spawn(move || worker_items.len());
    assert_eq!(worker.join().unwrap(), items.len());
}
```

## 2. Common mistake

Arc<RefCell<T>> does not make RefCell safe for concurrent sharing.

## Practice

Share immutable data across two workers and collect their read-only results.

## References

- [Official Rust documentation · ch15 00 smart pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)

---

[← Previous: `Rc<T>`](<../02 - Rc/README.md>) | [Back to Smart Pointers](<../README.md>) | [Next: `RefCell<T>` →](<../04 - RefCell/README.md>)
