# Concurrent web server

Build a concurrent local HTTP server for a small documented set of routes. Use a maintained HTTP stack for parsing, set request and concurrency limits, and separate pure handler logic from the listener.

## Contents

1. [Worked example](#1-worked-example)
2. [Design check](#2-design-check)

## 1. Worked example

```text
serve GET /health and GET /notes/{id} on loopback
test success, unknown routes, and overloaded handler limits
run concurrent clients while requesting shutdown
```

## 2. Design check

Bound request size and time spent waiting on slow clients. Keep shutdown behavior explicit when workers are servicing requests.

## Acceptance criteria

- Two clients can make progress concurrently.
- Requests over the documented limit are rejected consistently.
- Graceful shutdown drains accepted work up to a deadline.

## References

- [Official Rust documentation · ch02 00 guessing game tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

---

[Back to Advanced](<../README.md>) | [Next: Real-time chat application →](<../02 - Real-time chat application/README.md>)
