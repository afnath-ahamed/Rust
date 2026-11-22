# CLI applications

A CLI combines argument parsing, I/O, exit status, errors, and documentation. Design the command contract and keep core behavior callable from tests.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
tool search --path DIR PATTERN
stdout: results only
stderr: actionable diagnostics
exit status: documented for matches, empty output, and errors
```

## 2. Common mistake

Mixing results and diagnostics makes scripting and testing harder.

## Practice

Support paths with spaces and verify empty results, permission failures, and a successful search.

## References

- [rust-lang.org · learn](https://rust-lang.org/learn/)

---

[← Previous: Operating systems](<../05 - Operating systems/README.md>) | [Back to Specialization Paths](<../README.md>) | [Next: Desktop applications (Tauri) →](<../07 - Desktop applications (Tauri)/README.md>)
