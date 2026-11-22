# Drop

Rust runs destructors for owned values on normal scope exit. A Drop implementation defines cleanup for a type. std::mem::drop takes ownership so cleanup can happen at a chosen point.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    struct Marker<'a>(&'a std::cell::Cell<bool>);
    impl Drop for Marker<'_> {
        fn drop(&mut self) {
            self.0.set(true);
        }
    }
    let cleaned = std::cell::Cell::new(false);
    drop(Marker(&cleaned));
    assert!(cleaned.get());
}
```

## 2. Common mistake

An abort bypasses normal destruction, so Drop alone cannot guarantee an external transaction is durable.

## Practice

Create two markers and record the order in which their destructors run.

## Related lesson

[Interior mutability](<../../13 - Smart Pointers/10 - Interior mutability/README.md>)

## References

- [Official Rust documentation · ch04 01 what is ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

---

[← Previous: Return values and ownership](<../07 - Return values and ownership/README.md>) | [Back to Ownership System](<../README.md>) | [Next: Borrowing and References →](<../../05 - Borrowing and References/README.md>)
