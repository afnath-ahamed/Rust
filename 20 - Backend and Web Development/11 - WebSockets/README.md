# WebSockets

WebSockets provide a long-lived bidirectional message channel after an HTTP upgrade. Define a message protocol, limits, and a cleanup path for disconnected or slow clients.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
client → Join(room) → server
server → Joined(room) → client
either side disconnects → unregister subscription and release buffers
```

## 2. Common mistake

Unbounded per-client queues turn slow consumers into growing memory pressure.

## Practice

Define what a server does when a client cannot keep up and test using a local simulated client.

## References

- [docs.rs · ws](https://docs.rs/axum/latest/axum/extract/ws/)
- [docs.rs · axum](https://docs.rs/axum/latest/axum/)

---

[← Previous: Authorization](<../10 - Authorization/README.md>) | [Back to Backend and Web Development](<../README.md>) | [Next: gRPC →](<../12 - gRPC/README.md>)
