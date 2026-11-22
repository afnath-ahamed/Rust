# Cloud deployment

Cloud deployment starts from an artifact and operational requirements. Choose a hosting model by needed networking, persistence, deployment controls, and operational load. Document the rollback mechanism.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
build image → deploy reviewed digest to staging → check health and logs → promote same digest → observe
on failure: restore previous digest and evaluate any schema changes separately
```

## 2. Common mistake

An application rollback may be incompatible with a destructive database migration. Use compatible rollout sequencing.

## Practice

Write a release plan for an API adding one database column without breaking the previous version.

## References

- [docs.docker.com · multi stage](https://docs.docker.com/build/building/multi-stage/)

---

[← Previous: GitHub Actions](<../04 - GitHub Actions/README.md>) | [Back to Production and DevOps](<../README.md>) | [Next: Kubernetes →](<../06 - Kubernetes/README.md>)
