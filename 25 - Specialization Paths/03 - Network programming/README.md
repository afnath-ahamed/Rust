# Network programming

Networking joins reliable parsing with protocol framing, backpressure, timeouts, and observability. Practice on local sockets so both endpoints and failure conditions remain reproducible.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
Implement a length-prefixed local echo service. Set a maximum frame size before allocation, then read one complete frame at a time.
```

## 2. Common mistake

A single stream read is not guaranteed to return a complete application message.

## Practice

Test frames split across reads, two frames in one read, disconnects, and an oversized frame.

## References

- [rust-lang.org · learn](https://rust-lang.org/learn/)

---

[← Previous: Systems programming](<../02 - Systems programming/README.md>) | [Back to Specialization Paths](<../README.md>) | [Next: Embedded systems →](<../04 - Embedded systems/README.md>)
