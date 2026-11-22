# Integration testing

Integration tests live under tests and exercise the public API through a separate test crate. They catch wiring and interface problems that private unit tests cannot see.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
src/lib.rs       exposes inventory::reserve
tests/reserve.rs imports inventory::reserve and verifies the public result
```

```shell
cargo test --test reserve
```

## 2. Common mistake

A test that relies on the package's private implementation is often brittle at this boundary.

## Practice

Create one integration test that checks a rejected request leaves the public state unchanged.

## References

- [Official Rust documentation · ch11 00 testing](https://doc.rust-lang.org/book/ch11-00-testing.html)

---

[← Previous: Unit testing](<../01 - Unit testing/README.md>) | [Back to Testing and Debugging](<../README.md>) | [Next: Documentation testing →](<../03 - Documentation testing/README.md>)
