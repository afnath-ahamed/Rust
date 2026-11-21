# Functions

Functions group reusable behavior behind a name. A function can accept typed parameters, compute a result, and return that result to its caller. Rust checks every argument and return type at compile time.

## Contents

1. [Define and call a function](#1-define-and-call-a-function)
2. [Parameters](#2-parameters)
3. [Return values](#3-return-values)
4. [Statements and expressions](#4-statements-and-expressions)
5. [Scope inside functions](#5-scope-inside-functions)
6. [Useful function patterns](#6-useful-function-patterns)
7. [Best practices](#7-best-practices)

## 1. Define and call a function

Declare a function with `fn`, a name, parentheses, and a body. The `main` function is the program's entry point:

```rust
fn greet() {
    println!("Hello from Rust!");
}

fn main() {
    greet();
}
```

Rust uses `snake_case` for function names. A function must be defined or otherwise available before it is called; Rust does not require a particular order in the source file.

## 2. Parameters

Each parameter needs a name and an explicit type. Multiple parameters are separated by commas:

```rust
fn print_profile(name: &str, age: u32) {
    println!("{name} is {age} years old.");
}

fn main() {
    print_profile("Afnath", 25);
}
```

The argument types must match the parameter types. Rust does not automatically convert values at a function call; convert them explicitly when necessary.

## 3. Return values

Write `-> Type` after the parameter list to declare a return type. The final expression becomes the return value when it has no semicolon:

```rust
fn add(left: i32, right: i32) -> i32 {
    left + right
}

fn main() {
    let total = add(2, 3);
    println!("{total}");
}
```

Use `return` for an early return:

```rust
fn absolute(value: i32) -> i32 {
    if value >= 0 {
        return value;
    }

    -value
}
```

Functions without a useful result return the unit type `()`, usually written without an explicit return type.

## 4. Statements and expressions

Rust distinguishes statements, which perform an action, from expressions, which produce a value. A semicolon turns most expressions into statements:

```rust
fn square(value: i32) -> i32 {
    let result = value * value; // statement
    result                     // expression returned from the function
}
```

See [Expressions and Statements](<../10 - Expressions and Statements/README.md>) for the full rule set.

## 5. Scope inside functions

Parameters and local variables are available only inside the function or block where they are declared:

```rust
fn show_message() {
    let message = "inside the function";
    println!("{message}");
}

fn main() {
    show_message();
    // `message` is not available here.
}
```

An inner block can use names from its outer block, but the outer block cannot use names declared inside the inner block. Ownership also determines whether a value is moved into a function or borrowed by it.

## 6. Useful function patterns

### Borrow instead of taking ownership

Use a reference when a function only needs to read a value:

```rust
fn length(text: &String) -> usize {
    text.len()
}
```

In many APIs, `&str` is more flexible than `&String`:

```rust
fn announce(message: &str) {
    println!("{message}");
}
```

### Return a `Result`

Functions that can fail commonly return `Result<T, E>` so the caller must handle success or failure explicitly:

```rust
fn parse_count(text: &str) -> Result<u32, std::num::ParseIntError> {
    text.parse()
}
```

## 7. Best practices

- Give each function one clear responsibility.
- Use descriptive names and precise parameter types.
- Keep functions small enough to understand at a glance.
- Prefer the final expression for simple returns.
- Borrow data when the function does not need ownership.
- Return `Result` for operations that can fail instead of hiding errors.

## Practice

1. Write a function that prints a greeting.
2. Create a function that accepts two numbers and returns their product.
3. Write a function that returns the larger of two values.
4. Create a function that borrows a string slice and reports its length.
5. Write a parsing function that returns a `Result<u32, _>`.

## Source references

This lesson was informed by the [W3Schools Rust Functions tutorial](https://www.w3schools.com/rust/rust_functions.php) and [Rust Scope tutorial](https://www.w3schools.com/rust/rust_scope.php), then expanded with Rust-specific guidance on expressions, borrowing, and error-returning functions.

---

[← Previous: Operators](<../08 - Operators/README.md>) | [Back to Language Basics](../README.md) | [Next: Expressions and Statements →](<../10 - Expressions and Statements/README.md>)
