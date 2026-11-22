# Desktop applications (Tauri)

Tauri combines a webview frontend with Rust commands. Keep the interface between them typed and grant only the capabilities the app needs. Validate arguments before accessing local files or operating-system features.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
UI command → validated Rust input → application action → typed response
Keep allowed paths and capabilities scoped to the task.
```

## 2. Common mistake

Treat frontend command payloads as external input even when both components ship together.

## Practice

Build a small local notes app and test invalid input and disallowed paths.

## References

- [v2.tauri.app · start](https://v2.tauri.app/start/)
- [rust-lang.org · learn](https://rust-lang.org/learn/)

---

[← Previous: CLI applications](<../06 - CLI applications/README.md>) | [Back to Specialization Paths](<../README.md>) | [Next: WebAssembly (WASM) →](<../08 - WebAssembly (WASM)/README.md>)
