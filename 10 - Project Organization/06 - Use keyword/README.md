# Use keyword

use brings a path into lexical scope. as resolves naming clashes. pub use can present a stable public path through a facade.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    use std::collections::VecDeque as Queue;
    let mut jobs = Queue::new();
    jobs.push_back(7);
    assert_eq!(jobs.pop_front(), Some(7));
}
```

## 2. Common mistake

Heavy glob imports obscure which module defines an item.

## Practice

Replace a glob import with the exact symbols used by a module.

## References

- [Official Rust documentation · ch07 00 managing growing projects with packages crates and modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

[← Previous: Public and private visibility](<../05 - Public and private visibility/README.md>) | [Back to Project Organization](<../README.md>) | [Next: Library crates →](<../07 - Library crates/README.md>)
