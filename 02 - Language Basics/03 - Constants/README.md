# Constants

Constants store values that do not change during the program's execution. Declare them with `const`, give them an explicit type, and assign a value that Rust can evaluate at compile time.

## Contents

1. [Declare a constant](#declare-a-constant)
2. [Required type annotations](#required-type-annotations)
3. [Naming conventions](#naming-conventions)
4. [Use a constant](#use-a-constant)
5. [Constants versus variables](#constants-versus-variables)
6. [Constants versus static items](#constants-versus-static-items)

## Declare a constant

Use `const`, followed by an uppercase name, a type, and a value:

```rust
const MINUTES_PER_HOUR: u32 = 60;
const MAX_RETRIES: u8 = 3;
```

Constants are associated with the scope where they are declared. They can be declared at module level or inside a function.

## Required type annotations

Unlike a normal `let` binding, a constant must include its type:

```rust
const DAYS_PER_WEEK: u8 = 7;
```

This is incomplete and does not compile:

```rust,compile_fail
const DAYS_PER_WEEK = 7;
```

The value must also be usable in a compile-time constant expression. A runtime result, such as user input, cannot be assigned directly to a `const`.

## Naming conventions

Rust conventionally uses `SCREAMING_SNAKE_CASE` for constants:

```rust
const PI_APPROXIMATION: f64 = 3.14159;
const DEFAULT_TIMEOUT_SECONDS: u64 = 30;
```

The uppercase style is a convention that makes constants easy to recognize. Rust may warn when a constant does not follow the convention.

## Use a constant

Constants can be used anywhere a value of the declared type is expected:

```rust
const MINUTES_PER_HOUR: u32 = 60;

fn main() {
    let working_minutes = 8 * MINUTES_PER_HOUR;
    println!("A working day has {working_minutes} minutes.");
}
```

Output:

```text
A working day has 480 minutes.
```

## Constants versus variables

| Feature | Constant (`const`) | Variable (`let`) |
| --- | --- | --- |
| Value can change | No | No by default, yes with `mut` |
| Type annotation | Required | Often inferred |
| Value evaluation | Compile-time constant expression | May be computed at runtime |
| Naming convention | Usually `SCREAMING_SNAKE_CASE` | Usually `snake_case` |
| Declaration scope | Module, block, or associated scope | A local or pattern binding |

```rust
const MAX_SCORE: u32 = 100;

fn main() {
    let mut score = 0;
    score += 10;

    println!("{score} out of {MAX_SCORE}");
}
```

Use `const` for a named value that never changes. Use `let` for a binding whose value belongs to a particular execution and may need to be mutable.

## Constants versus static items

Both `const` and `static` values do not change, but they have different purposes:

- A `const` represents a value that may be copied or inlined wherever it is used.
- A `static` represents one fixed memory location for the entire program and can be referenced by address.
- `static mut` requires unsafe access and should be avoided unless a low-level design requires it.

Use `const` for ordinary configuration values and simple named limits. Learn `static` separately when working with global state or program-wide storage.

## Practice

1. Declare a constant for the number of seconds in a minute.
2. Use it to calculate the number of seconds in an hour.
3. Try declaring a constant without a type and read the compiler error.
4. Try assigning a new value to a constant and compare the result with changing a `let mut` variable.

## Source reference

This lesson was informed by the [W3Schools Rust Constants tutorial](https://www.w3schools.com/rust/rust_constants.php) and expanded with Rust-specific guidance on compile-time expressions, scope, and `static` items.

---

[← Previous: Mutability](<../02 - Mutability/README.md>) | [Back to Language Basics](../README.md) | [Next: Shadowing →](<../04 - Shadowing/README.md>)
