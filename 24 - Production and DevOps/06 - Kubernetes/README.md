# Kubernetes

Kubernetes reconciles workloads toward declared state. A Deployment manages replicated Pods and a Service provides stable access. Configure resources and readiness around the application's real behavior.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
Deployment: image digest, replicas, resource requests, readiness probe
Service: selector matching pod labels and target port
Rollout: observe readiness and run a smoke check
```

## 2. Common mistake

A Pod local filesystem is normally disposable. Store durable state in a suitable external or persistent system.

## Practice

Write a minimal manifest for a local service and verify that the Service selects only expected Pods.

## References

- [docs.docker.com · multi stage](https://docs.docker.com/build/building/multi-stage/)

---

[← Previous: Cloud deployment](<../05 - Cloud deployment/README.md>) | [Back to Production and DevOps](<../README.md>) | [Next: Observability →](<../07 - Observability/README.md>)
