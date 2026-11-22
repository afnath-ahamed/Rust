# Copy semantics

A type implementing Copy can be duplicated implicitly in value contexts. Many scalar types are Copy, and a compound type can be Copy when its components permit it.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let primary = (3u8, true);
    let secondary = primary;
    assert_eq!(primary, secondary);
}
```

## 2. Common mistake

Types that manage resources with a destructor cannot implement Copy.

## Practice

Try substituting a String for the first field and explain the change in behavior.

## References

- [Official Rust documentation · ch04 01 what is ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

---

[← Previous: Move semantics](<../03 - Move semantics/README.md>) | [Back to Ownership System](<../README.md>) | [Next: Clone →](<../05 - Clone/README.md>)
