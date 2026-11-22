# Executors

An executor polls tasks that are ready to make progress. A waker can schedule a task again. Reactors or other event sources tell the runtime when I/O or time is ready.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
1. Schedule task and poll its future.
2. On Pending, continue with another ready task.
3. On wake, mark the original task ready for another poll.
4. On Ready, deliver the output and finish the task.
```

## 2. Common mistake

A waker requests another poll and does not guarantee immediate execution.

## Practice

Trace the task state for a future that is pending once and then resolves.

## References

- [Official Rust documentation · ch17 00 async await](https://doc.rust-lang.org/book/ch17-00-async-await.html)

---

[← Previous: Futures](<../04 - Futures/README.md>) | [Back to Asynchronous Programming](<../README.md>) | [Next: Async runtimes →](<../06 - Async runtimes/README.md>)
