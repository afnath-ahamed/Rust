# Procedural macros

Procedural macros transform token streams at compilation. Derive macros add items for an input type, attribute macros can transform items, and function-like macros take token input at their invocation.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```toml
[lib]
proc-macro = true
```

```text
workspace/
├── derive_impl/   # token parsing and expansion
└── consumer/      # uses and tests expansion via the public API
```

## 2. Common mistake

Procedural macros execute during builds. Treat their dependencies as code with build-time access.

## Practice

Design a derive that emits a single trait implementation and test invalid input in a consumer crate.

## Related lesson

[Public and private visibility](<../../10 - Project Organization/05 - Public and private visibility/README.md>)

## References

- [Official Rust documentation · ch20 00 advanced features](https://doc.rust-lang.org/book/ch20-00-advanced-features.html)

---

[← Previous: Declarative macros](<../11 - Declarative macros/README.md>) | [Back to Advanced Rust](<../README.md>) | [Next: Foreign Function Interface (FFI) →](<../13 - Foreign Function Interface (FFI)/README.md>)
