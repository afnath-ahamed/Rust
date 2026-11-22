# Public and private visibility

Items are private by default. pub exposes an item where visibility permits, and pub(crate) limits access to its crate. A public struct can still protect its fields.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
mod inventory {
    pub struct Stock(u32);
    impl Stock {
        pub fn new(count: u32) -> Self {
            Self(count)
        }
        pub fn count(&self) -> u32 {
            self.0
        }
    }
}
assert_eq!(inventory::Stock::new(7).count(), 7);
```

## 2. Common mistake

A public child inside a private module may need a public re-export to make its path reachable from outside the crate.

## Practice

Expose a small type through the root while keeping its implementation module private.

## References

- [Official Rust documentation · ch07 00 managing growing projects with packages crates and modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

[← Previous: Module tree](<../04 - Module tree/README.md>) | [Back to Project Organization](<../README.md>) | [Next: Use keyword →](<../06 - Use keyword/README.md>)
