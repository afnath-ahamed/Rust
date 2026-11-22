# Modules

A mod declaration introduces a module. Its contents can be inline or loaded from the corresponding source file. Paths inside the crate can begin with crate, self, or super.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
mod inventory {
    pub fn count() -> usize {
        2
    }
}
assert_eq!(inventory::count(), 2);
```

## 2. Common mistake

use and mod solve different problems. Declaring a module does not automatically shorten paths.

## Practice

Move an inline module into its own file in a small Cargo package.

## References

- [Official Rust documentation · ch07 00 managing growing projects with packages crates and modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

[← Previous: Crates](<../02 - Crates/README.md>) | [Back to Project Organization](<../README.md>) | [Next: Module tree →](<../04 - Module tree/README.md>)
