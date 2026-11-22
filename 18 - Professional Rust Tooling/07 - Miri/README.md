# Miri

Miri interprets supported Rust programs and tests and can detect some forms of undefined behavior on executed paths. It usually runs on nightly and supports a limited set of platform interactions.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
rustup component add miri --toolchain nightly
cargo +nightly miri test
```

## 2. Common mistake

Passing Miri provides evidence for executed cases and does not establish soundness for all possible inputs.

## Practice

Run Miri on a small crate's pure logic tests and document any unsupported operation.

## References

- [github.com · miri](https://github.com/rust-lang/miri)
- [Official Rust documentation · index](https://doc.rust-lang.org/cargo/commands/index.html)

---

[← Previous: Cargo Deny](<../06 - Cargo Deny/README.md>) | [Back to Professional Rust Tooling](<../README.md>) | [Next: Compiler diagnostics →](<../08 - Compiler diagnostics/README.md>)
