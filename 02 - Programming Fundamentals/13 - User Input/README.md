# User Input (`std::io`)

## Contents

1. [What is user input?](#1-what-is-user-input)
2. [Basic user input](#2-basic-user-input)
3. [Type conversion with `parse()`](#3-type-conversion-with-parse)
4. [Error handling](#4-error-handling)
5. [Multiple inputs](#5-multiple-inputs)
6. [`print!()` versus `println!()`](#6-print-versus-println)
7. [Complete example](#7-complete-example--add-two-numbers)

## 1. What is user input?

User input allows a Rust program to receive data from the user during execution.

- `std::io` — standard input/output module.
- `stdin()` — reads input from the terminal.
- `stdout()` — writes output to the terminal.
- **Important:** `read_line()` reads input as a `String`, even when the user enters a number.

### Basic input flow

![User input flow](user-input-flow.png)

## 2. Basic user input

```rust
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    println!("Input: {}", input.trim());
}
```

| Component | Purpose |
| --- | --- |
| `use std::io;` | Imports standard input/output. |
| `String::new()` | Creates an empty string. |
| `mut` | Allows the input buffer to be modified. |
| `&mut input` | Passes a mutable reference. |
| `read_line()` | Reads one line from the terminal. |
| `trim()` | Removes surrounding whitespace and the newline. |
| `expect()` | Stops with a useful message if reading fails. |

## 3. Type conversion with `parse()`

Input is text. Convert it explicitly:

```rust
let age: u32 = input.trim().parse().expect("Invalid age");
```

**Flow:** Input (`String`) → `trim()` → `parse()` → Number (`u32`).

Common types: `i32`, `u32`, `f32`, `f64`, and `bool`.

## 4. Error handling

Parsing returns a `Result`:

```rust
match input.trim().parse::<i32>() {
    Ok(number) => println!("Number: {number}"),
    Err(_) => println!("Invalid input"),
}
```

| Method | Behavior |
| --- | --- |
| `unwrap()` | Returns the value or panics. |
| `expect("message")` | Returns the value or panics with a message. |
| `match` | Handles success and failure explicitly. |

## 5. Multiple inputs

Clear and reuse the same buffer:

```rust
input.clear();
io::stdin().read_line(&mut input).unwrap();
```

Read space-separated values:

```rust
let numbers: Vec<i32> = input
    .split_whitespace()
    .map(|value| value.parse().expect("Invalid number"))
    .collect();
```

## 6. `print!()` versus `println!()`

| Macro | Purpose |
| --- | --- |
| `print!()` | Output without a newline. |
| `println!()` | Output with a newline. |
| `flush()` | Ensures buffered output is displayed. |

Flush buffered output when displaying a prompt:

```rust
use std::io::{self, Write};

print!("Enter your name: ");
io::stdout().flush().unwrap();
```

## 7. Complete example — add two numbers

```rust
use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter first number:");
    io::stdin().read_line(&mut input).unwrap();
    let a: i32 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter second number:");
    io::stdin().read_line(&mut input).unwrap();
    let b: i32 = input.trim().parse().unwrap();

    println!("Sum = {}", a + b);
}
```

**Output:** `10 + 20 = 30` (shown as `Sum = 30`).

## Key points to remember

- `read_line()` reads terminal input as text.
- Use `trim()` before parsing.
- Use `parse()` for numeric or Boolean conversion.
- Use `match` for safer error handling.
- `read_line()` appends input; `clear()` resets the string.
- Use a `loop` to request input repeatedly until valid.

Overall flow: user input → `stdin()` → `read_line()` → `trim()` → `parse()` → processing → output.

---

[← Previous: Control Flow](<../12 - Control Flow/README.md>) | [Back to Programming Fundamentals](../README.md) | [Next: Memory Management Fundamentals →](<../../03 - Memory Management Fundamentals/README.md>)
