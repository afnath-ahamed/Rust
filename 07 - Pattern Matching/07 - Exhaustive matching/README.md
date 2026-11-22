# Exhaustive matching

Exhaustive matching means every value has a reachable arm. Explicit enum arms can make a new variant produce compile errors at consumers that must handle it.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
enum Signal {
    Stop,
    Go,
}
fn instruction(signal: Signal) -> &'static str {
    match signal {
        Signal::Stop => "wait",
        Signal::Go => "move",
    }
}
assert_eq!(instruction(Signal::Stop), "wait");
assert_eq!(instruction(Signal::Go), "move");
```

## 2. Common mistake

A wildcard future-proofs syntax while potentially hiding an intended behavior change.

## Practice

Add a Slow variant and update each match the compiler identifies.

## References

- [Official Rust documentation · ch19 00 patterns](https://doc.rust-lang.org/book/ch19-00-patterns.html)

---

[← Previous: Match guards](<../06 - Match guards/README.md>) | [Back to Pattern Matching](<../README.md>) | [Next: Collections and Strings →](<../../08 - Collections and Strings/README.md>)
