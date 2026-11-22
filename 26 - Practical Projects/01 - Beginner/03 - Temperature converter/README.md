# Temperature converter

Build a converter between Celsius and Fahrenheit. Parse the scale arguments separately from the numeric value, and format only at the output boundary.

## Contents

1. [Rust example](#1-rust-example)
2. [Design check](#2-design-check)

## 1. Rust example

```rust
fn fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}
fn celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
assert_eq!(fahrenheit(0.0), 32.0);
assert!((celsius(fahrenheit(37.0)) - 37.0).abs() < 1e-9);
```

## 2. Design check

Decide whether nonfinite inputs are supported. For this project reject them, and compare nontrivial float results using a tolerance.

## Acceptance criteria

- Cover freezing, boiling, and negative values in both directions.
- Reject NaN, infinities, and unknown scale names.
- Document output precision and test round-trip behavior.

## Run the example project

From this folder, run the included [converter implementation](./src/main.rs).

```powershell
cargo run -- 0 C
cargo run -- 212 F
cargo run -- NaN C
cargo test
```

The first two invocations print `32.00 F` and `100.00 C`. The third writes
`temperature must be finite` to standard error and exits with a failure status.
Values are displayed to two decimal places.

## References

- [Official Rust documentation · ch02 00 guessing game tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

---

[← Previous: Guessing game](<../02 - Guessing game/README.md>) | [Back to Beginner](<../README.md>) | [Next: CLI task manager →](<../04 - CLI task manager/README.md>)
