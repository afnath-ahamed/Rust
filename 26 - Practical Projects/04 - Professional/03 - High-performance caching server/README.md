# High-performance caching server

Build a local cache server with bounded memory, expiry, deletion, and a documented eviction policy. Use a deliberately limited protocol and separate state handling from the transport.

## Contents

1. [Worked example](#1-worked-example)
2. [Design check](#2-design-check)

## 1. Worked example

```text
commands: GET key, SET key ttl value, DEL key
maintain entry limits and current byte use
measure latency, throughput, and memory under mixed workloads
```

## 2. Design check

Bound key sizes, value sizes, queue capacity, and stale metadata. Use a fake clock to test expiration and separate protocol failures from missing values.

## Acceptance criteria

- Expired entries are invisible and capacity is enforced.
- Parallel clients observe the documented operation semantics.
- Report memory and latency for a reproducible hot-key workload.

## References

- [Official Rust documentation · ch02 00 guessing game tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

---

[← Previous: Payment processing system](<../02 - Payment processing system/README.md>) | [Back to Professional](<../README.md>) | [Next: Distributed microservices system →](<../04 - Distributed microservices system/README.md>)
