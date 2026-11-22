# Embedded systems

Embedded Rust involves target-specific peripherals, limited resources, and often no_std. Pick a supported board or emulator, separate hardware dependencies from pure logic, and document reproducible builds.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
Milestones: choose target and HAL → blink LED or simulate output → sample input → add a timed state machine.
```

## 2. Common mistake

Host tests verify portable logic, while actual timing and peripherals need representative target checks.

## Practice

Implement a hardware-independent sensor alert state machine and document how you will validate it on target.

## References

- [docs.rust-embedded.org · book](https://docs.rust-embedded.org/book/)
- [rust-lang.org · learn](https://rust-lang.org/learn/)

---

[← Previous: Network programming](<../03 - Network programming/README.md>) | [Back to Specialization Paths](<../README.md>) | [Next: Operating systems →](<../05 - Operating systems/README.md>)
