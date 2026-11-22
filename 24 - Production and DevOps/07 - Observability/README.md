# Observability

Observability uses logs, metrics, and traces to explain what a system is doing. Correlate telemetry with stable request identifiers while keeping labels bounded and sensitive fields out of reports.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
Metrics: request rate, failures, and latency by route template
Traces: request span, database span, outgoing call span
Logs: actionable failures and trace IDs
```

## 2. Common mistake

Using user IDs or raw paths as metric labels can overwhelm the telemetry backend.

## Practice

Define a dashboard and a trace view for diagnosing a slow dependency.

## Related lesson

[Tracing](<../../17 - Testing and Debugging/10 - Tracing/README.md>)

## References

- [opentelemetry.io · docs](https://opentelemetry.io/docs/)
- [docs.docker.com · multi stage](https://docs.docker.com/build/building/multi-stage/)

---

[← Previous: Kubernetes](<../06 - Kubernetes/README.md>) | [Back to Production and DevOps](<../README.md>) | [Next: Health checks →](<../08 - Health checks/README.md>)
