# Drop trait

Drop lets an owning type release a resource at destruction. Keep cleanup predictable and avoid a panic in Drop, particularly during an existing unwind.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    struct Lease<'a>(&'a std::cell::Cell<bool>);
    impl Drop for Lease<'_> {
        fn drop(&mut self) {
            self.0.set(false);
        }
    }
    let held = std::cell::Cell::new(true);
    {
        let _lease = Lease(&held);
    }
    assert!(!held.get());
}
```

## 2. Common mistake

Errors that users must act on need an explicit close or flush method.

## Practice

Design a writer with a fallible finish method and best-effort cleanup in Drop.

## Related lesson

[Drop](<../../04 - Ownership System/08 - Drop/README.md>)

## References

- [Official Rust documentation · ch15 00 smart pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)

---

[← Previous: Deref trait](<../08 - Deref trait/README.md>) | [Back to Smart Pointers](<../README.md>) | [Next: Interior mutability →](<../10 - Interior mutability/README.md>)
