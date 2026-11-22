# Docker

Multi-stage Docker builds separate compilation from the runtime image. Match target architecture and runtime linkage, copy only needed files, and run as a limited user.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```dockerfile
FROM rust:bookworm AS build
WORKDIR /src
COPY . .
RUN cargo build --release --locked
FROM debian:bookworm-slim
COPY --from=build /src/target/release/app /usr/local/bin/app
USER 65532:65532
ENTRYPOINT ["/usr/local/bin/app"]
```

## 2. Common mistake

This template assumes a Cargo binary named app and a committed lockfile. Pin reviewed image versions or digests for reproducibility and include required runtime libraries.

## Practice

Build locally for a simple app and check startup, writable paths, and effective user.

## References

- [docs.docker.com · multi stage](https://docs.docker.com/build/building/multi-stage/)

---

[← Previous: Environment configuration](<../01 - Environment configuration/README.md>) | [Back to Production and DevOps](<../README.md>) | [Next: CI/CD →](<../03 - CI and CD/README.md>)
