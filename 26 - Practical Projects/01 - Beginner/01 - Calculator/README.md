# Calculator

Build a calculator that accepts two operands and an operation. Keep argument parsing separate from arithmetic so overflow, invalid operators, and division by zero each map to a meaningful error.

## Contents

1. [Rust example](#1-rust-example)
2. [Design check](#2-design-check)

## 1. Rust example

```rust
fn calculate(op: &str, a: i64, b: i64) -> Option<i64> {
    match op {
        "+" => a.checked_add(b),
        "-" => a.checked_sub(b),
        "*" => a.checked_mul(b),
        "/" => a.checked_div(b),
        _ => None,
    }
}
assert_eq!(calculate("*", 6, 7), Some(42));
assert_eq!(calculate("/", 8, 0), None);
```

## 2. Design check

The core example uses Option for brevity. Define a CalcError enum for the CLI so overflow, divide-by-zero, and bad operators have distinct messages. Shape the interface as cargo run -- 6 '*' 7 for predictable shell behavior.

## Acceptance criteria

- Valid arithmetic prints a single answer.
- Missing arguments, an unsupported operator, zero division, and overflow return documented errors.
- Test operand extremes and include three usage examples.

## Run the example project

From this folder, run the included [calculator implementation](./src/main.rs).

```powershell
cargo run -- 6 '*' 7
cargo run -- -7 / 2
cargo run -- 8 / 0
cargo test
```

The first two invocations print `42` and `-3`. The third writes
`cannot divide by zero` to standard error and exits with a failure status.
Division uses integer arithmetic and truncates toward zero.

## References

- [Official Rust documentation · ch02 00 guessing game tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

---

[Back to Beginner](<../README.md>) | [Next: Guessing game →](<../02 - Guessing game/README.md>)
