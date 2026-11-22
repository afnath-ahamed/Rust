# While let

A while let loop repeats while an expression matches its pattern. It is useful when successive operations return an Option describing whether a value is available.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    let mut stack = vec![1, 2, 3];
    let mut popped = Vec::new();
    while let Some(item) = stack.pop() {
        popped.push(item);
    }
    assert_eq!(popped, [3, 2, 1]);
}
```

## 2. Common mistake

Ensure the loop changes the state being tested or has a deliberate exit path.

## Practice

Drain a VecDeque from its front and compare the order with Vec::pop.

## References

- [Official Rust documentation · ch19 00 patterns](https://doc.rust-lang.org/book/ch19-00-patterns.html)

---

[← Previous: If let](<../02 - If let/README.md>) | [Back to Pattern Matching](<../README.md>) | [Next: Let else →](<../04 - Let else/README.md>)
