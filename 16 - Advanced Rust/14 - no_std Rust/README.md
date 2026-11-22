# no_std Rust

no_std removes the automatic dependency on std. Use core for language fundamentals and optionally alloc when the environment supplies allocation support. Entry points and platform services are target-specific.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```rust
#![no_std]
pub fn rotate(value: u8) -> u8 {
    value.rotate_left(1)
}

#[cfg(test)]
mod tests {
    #[test]
    fn wraps_the_high_bit() {
        assert_eq!(super::rotate(0b1000_0001), 0b0000_0011);
    }
}
```

Put the example in `src/lib.rs` of a library package and run `cargo test`
on the host. Run `cargo check --target <target-triple>` to check the library
for a target you have installed.

## 2. Common mistake

A dependency can reintroduce std through default features. Audit its features for the intended target.

## Practice

Create a no_std library crate, then verify it compiles on a supported embedded target.

## References

- [Official Rust documentation · ch20 00 advanced features](https://doc.rust-lang.org/book/ch20-00-advanced-features.html)

---

[← Previous: Foreign Function Interface (FFI)](<../13 - Foreign Function Interface (FFI)/README.md>) | [Back to Advanced Rust](<../README.md>) | [Next: Testing and Debugging →](<../../17 - Testing and Debugging/README.md>)
