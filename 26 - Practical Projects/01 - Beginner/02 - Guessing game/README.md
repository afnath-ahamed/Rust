# Guessing game

Build a game that repeatedly reads a guess and compares it to a hidden target. Inject the target into the game logic to make tests deterministic. Add random target generation at the application boundary.

## Contents

1. [Rust example](#1-rust-example)
2. [Design check](#2-design-check)

## 1. Rust example

```rust
fn hint(target: u8, guess: u8) -> &'static str {
    match guess.cmp(&target) {
        std::cmp::Ordering::Less => "higher",
        std::cmp::Ordering::Greater => "lower",
        std::cmp::Ordering::Equal => "correct",
    }
}
assert_eq!(hint(42, 41), "higher");
assert_eq!(hint(42, 42), "correct");
```

## 2. Design check

Choose whether invalid input counts as an attempt, and exit cleanly on EOF. Keep target generation separate from the comparison and prefer a maintained randomness library as shown in the Rust Book's guessing game chapter.

## Acceptance criteria

- Read unsigned inputs within a documented range.
- Return higher, lower, or correct for deterministic test cases.
- Handle invalid text, out-of-range values, and EOF without busy-looping.

## References

- [Official Rust documentation · ch02 00 guessing game tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

---

[← Previous: Calculator](<../01 - Calculator/README.md>) | [Back to Beginner](<../README.md>) | [Next: Temperature converter →](<../03 - Temperature converter/README.md>)
