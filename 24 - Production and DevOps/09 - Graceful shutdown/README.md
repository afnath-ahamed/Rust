# Graceful shutdown

Graceful shutdown stops admitting new work, signals background tasks, drains or safely cancels active work, and exits within a deadline. Record what happens to unfinished work.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```rust no_run
async fn shutdown_signal() {
    if let Err(error) = tokio::signal::ctrl_c().await {
        eprintln!("could not wait for shutdown signal: {error}");
    }
}
```

## 2. Common mistake

This Ctrl-C example uses Tokio's signal feature. Container platforms often send SIGTERM and require a corresponding platform handler.

## Practice

Test shutdown with active requests, a slow worker, and a task that hits the shutdown deadline.

## References

- [docs.docker.com · multi stage](https://docs.docker.com/build/building/multi-stage/)

---

[← Previous: Health checks](<../08 - Health checks/README.md>) | [Back to Production and DevOps](<../README.md>) | [Next: Production monitoring →](<../10 - Production monitoring/README.md>)
