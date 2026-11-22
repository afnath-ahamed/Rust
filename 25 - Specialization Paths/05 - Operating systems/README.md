# Operating systems

Operating-system development adds bootstrapping, memory mapping, interrupts, and hardware boundaries. Start in an emulator and keep each unsafe interface small and documented.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
Milestones: boot in emulator → write a serial message → test a small page allocator → add a local trap handler.
```

## 2. Common mistake

Kernel memory safety depends on invariants enforced across architecture-specific boundaries.

## Practice

Create an emulator smoke test and document the conditions required by one low-level abstraction.

## References

- [rust-lang.org · learn](https://rust-lang.org/learn/)

---

[← Previous: Embedded systems](<../04 - Embedded systems/README.md>) | [Back to Specialization Paths](<../README.md>) | [Next: CLI applications →](<../06 - CLI applications/README.md>)
