# Health checks

Liveness answers whether a process should be restarted. Readiness answers whether an instance should receive traffic now. Startup probes allow slow initialization before normal checks take effect.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
GET /live  -> process can make progress
GET /ready -> instance can currently serve requests
startup probe -> initialization completed
```

## 2. Common mistake

Making liveness depend on a shared database can cause restart loops during a database outage.

## Practice

Write a probe policy for an API that can stay alive when its database is unavailable.

## References

- [docs.docker.com · multi stage](https://docs.docker.com/build/building/multi-stage/)

---

[← Previous: Observability](<../07 - Observability/README.md>) | [Back to Production and DevOps](<../README.md>) | [Next: Graceful shutdown →](<../09 - Graceful shutdown/README.md>)
