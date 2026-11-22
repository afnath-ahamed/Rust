# CLI task manager

Build a command-line task manager with add, list, complete, and remove commands. Start with a Vec of records and stable task IDs, then persist the collection locally.

## Contents

1. [Rust example](#1-rust-example)
2. [Design check](#2-design-check)

## 1. Rust example

```rust
fn main() {
    struct Task {
        id: u64,
        done: bool,
    }
    fn complete(tasks: &mut [Task], id: u64) -> bool {
        if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
            task.done = true;
            true
        } else {
            false
        }
    }
    let mut tasks = [Task { id: 1, done: false }];
    assert!(complete(&mut tasks, 1));
    assert!(tasks[0].done);
}
```

## 2. Design check

Separate the backing format from printing. Validate on load and design updates so an interrupted write does not leave a partially written task list. Document the platform behavior of replacement.

## Acceptance criteria

- Add and list tasks across restarts.
- Completing an unknown ID has a defined error and cannot affect another task.
- Tests cover empty state, duplicate completion, and damaged data.

## References

- [Official Rust documentation · ch02 00 guessing game tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

---

[← Previous: Temperature converter](<../03 - Temperature converter/README.md>) | [Back to Beginner](<../README.md>) | [Next: Intermediate →](<../../02 - Intermediate/README.md>)
