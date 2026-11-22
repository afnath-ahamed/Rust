# WebAssembly (WASM)

WebAssembly lets compiled Rust run inside a host with explicit interface capabilities. Browser integration often uses bindings, and server or embedded runtimes may have different host imports.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
Compile a pure text transformation for WebAssembly, build minimal host bindings, then compare behavior with native tests.
```

## 2. Common mistake

Crossing the host boundary and serializing data can dominate the cost of small operations.

## Practice

Test the same fixtures in native Rust and in the WebAssembly host, then measure artifact size.

## References

- [rustwasm.github.io · book](https://rustwasm.github.io/docs/book/)
- [rust-lang.org · learn](https://rust-lang.org/learn/)

---

[← Previous: Desktop applications (Tauri)](<../07 - Desktop applications (Tauri)/README.md>) | [Back to Specialization Paths](<../README.md>) | [Next: Blockchain →](<../09 - Blockchain/README.md>)
