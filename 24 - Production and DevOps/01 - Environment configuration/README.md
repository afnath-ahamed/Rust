# Environment configuration

Read configuration at startup and validate it into one typed structure. Separate public settings from secrets, define precedence, and fail with clear diagnostics when required settings are missing.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn parse_workers(input: &str) -> Option<usize> {
    input.parse().ok().filter(|n| (1..=64).contains(n))
}
assert_eq!(parse_workers("8"), Some(8));
assert_eq!(parse_workers("0"), None);
```

## 2. Common mistake

Do not print an entire unredacted configuration structure at startup.

## Practice

Define precedence for defaults, configuration files, and environment settings, then test four override cases.

## References

- [docs.docker.com · multi stage](https://docs.docker.com/build/building/multi-stage/)

---

[← Previous: Security Engineering](<../../23 - Security Engineering/README.md>) | [Back to Production and DevOps](<../README.md>) | [Next: Docker →](<../02 - Docker/README.md>)
