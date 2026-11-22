# Real-time chat application

Build a chat app with rooms and a bounded message protocol. Use a central room registry and clean up registrations when clients disconnect. Define message order within each room.

## Contents

1. [Worked example](#1-worked-example)
2. [Design check](#2-design-check)

## 1. Worked example

```text
client emits Join, Send, and Leave
server emits Joined, Message, Rejected, and a disconnect outcome
each client has bounded outbound buffering
```

## 2. Design check

A slow receiver must not stall every other client in the room. Pick a clear policy for dropping messages or disconnecting slow clients.

## Acceptance criteria

- Two local clients exchange Unicode text in one room.
- Room isolation, reconnect behavior, and invalid messages are tested.
- Disconnected clients leave no registered subscriptions.

## References

- [Official Rust documentation · ch02 00 guessing game tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

---

[← Previous: Concurrent web server](<../01 - Concurrent web server/README.md>) | [Back to Advanced](<../README.md>) | [Next: Tauri desktop application →](<../03 - Tauri desktop application/README.md>)
