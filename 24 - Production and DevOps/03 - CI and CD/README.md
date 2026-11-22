# CI/CD

CI checks changes automatically. Continuous delivery packages verified revisions for release, and continuous deployment can promote them automatically under a defined policy.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
pull request: format → lint → test
approved revision: build image → record digest → deploy staging → smoke test → promote digest
```

## 2. Common mistake

Rebuilding independently for each environment weakens the link between tested and deployed artifacts.

## Practice

Define the exact checks that can block promotion and how to roll back a bad release.

## References

- [docs.docker.com · multi stage](https://docs.docker.com/build/building/multi-stage/)

---

[← Previous: Docker](<../02 - Docker/README.md>) | [Back to Production and DevOps](<../README.md>) | [Next: GitHub Actions →](<../04 - GitHub Actions/README.md>)
