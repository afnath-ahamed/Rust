# GitHub Actions

GitHub Actions runs repository workflows on events. Keep permissions minimal, use explicit commands, and protect secrets in trusted release jobs. Pin third-party actions to reviewed revisions when appropriate.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```yaml
name: Rust checks
on: [push, pull_request]
permissions:
  contents: read
jobs:
  checks:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: rustup component add rustfmt clippy
      - run: cargo fmt --all -- --check
      - run: cargo clippy --locked --all-targets -- -D warnings
      - run: cargo test --locked
```

## 2. Common mistake

This example assumes a Cargo project or workspace at the repository root. For nested projects, set an appropriate working directory for run steps.

## Practice

Adapt the workflow to a nested Cargo package and verify the checks run in the intended directory.

## References

- [docs.github.com · actions](https://docs.github.com/en/actions)
- [docs.docker.com · multi stage](https://docs.docker.com/build/building/multi-stage/)

---

[← Previous: CI/CD](<../03 - CI and CD/README.md>) | [Back to Production and DevOps](<../README.md>) | [Next: Cloud deployment →](<../05 - Cloud deployment/README.md>)
