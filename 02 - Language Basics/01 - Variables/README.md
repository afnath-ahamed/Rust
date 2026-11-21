# Variables

Variables are named bindings that let a Rust program keep and use values. This lesson covers declaration, use, naming, and type annotations. Mutability and shadowing have dedicated lessons so the same material is not repeated here.

## Contents

1. [Declare and use a variable](#declare-and-use-a-variable)
2. [Naming bindings](#naming-bindings)
3. [Type annotations](#type-annotations)
4. [Next lessons](#next-lessons)

## Declare and use a variable

Use `let` to create a binding, then use its name in an expression:

```rust
fn main() {
    let name = "John";
    let age = 30;

    println!("{name} is {age} years old.");
}
```

Rust usually infers each binding's type from its value. Captured formatting such as `{name}` inserts the value into the output.

## Naming bindings

Use clear `snake_case` names that describe the value. A binding must be declared before it is used, and each name follows Rust's normal scope rules.

```rust
let first_name = "John";
let account_balance = 1250;
```

## Type annotations

Add `: Type` after a name when the intended type should be explicit or inference needs help:

```rust
fn main() {
    let count: i32 = 30;
    let initial: char = 'J';

    println!("{initial} represents {count}.");
}
```

The annotation does not convert a value; it tells the compiler which type the binding must have. Type conversion is covered in [Type Conversion](<../07 - Type Conversion/README.md>).

## Next lessons

- [Mutability](<../02 - Mutability/README.md>) — change a binding with `mut`.
- [Constants](<../03 - Constants/README.md>) — define named compile-time values with `const`.
- [Shadowing](<../04 - Shadowing/README.md>) — create a new binding with an existing name.

## Source reference

This lesson was informed by the [W3Schools Rust Variables tutorial](https://www.w3schools.com/rust/rust_variables.php) and expanded with Rust-specific guidance.
