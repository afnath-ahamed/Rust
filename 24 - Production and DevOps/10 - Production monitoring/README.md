# Production monitoring

Production monitoring continually checks important user outcomes. Set objectives for latency and availability, couple alerts to actionable symptoms, and keep ownership and recovery instructions close at hand.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
Symptom: sustained increase in failed requests
Correlate: recent release, queue depth, dependency errors, and saturation
Response: diagnose, mitigate, validate recovery, and record incident details
```

## 2. Common mistake

An alert for every transient error can hide the sustained problems operators need to act on.

## Practice

Define an alert and one runbook that establishes when to roll back and how to confirm recovery.

## References

- [docs.docker.com · multi stage](https://docs.docker.com/build/building/multi-stage/)

---

[← Previous: Graceful shutdown](<../09 - Graceful shutdown/README.md>) | [Back to Production and DevOps](<../README.md>) | [Next: Specialization Paths →](<../../25 - Specialization Paths/README.md>)
