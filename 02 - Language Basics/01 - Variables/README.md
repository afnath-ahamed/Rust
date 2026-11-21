# Variables

Variables hold values that a Rust program can use. Create a variable with `let`. Rust bindings are immutable by default, which makes accidental changes easier to prevent.

## Contents

1. [Create a variable](#create-a-variable)
2. [Use a variable](#use-a-variable)
3. [Immutability by default](#immutability-by-default)
4. [Make a variable mutable](#make-a-variable-mutable)
5. [Add a type annotation](#add-a-type-annotation)
6. [Variable scope](#variable-scope)

## Create a variable

Use `let` followed by a name and a value:

```rust
let name = "John";
```

The compiler infers that `name` is a string slice because of the value assigned to it.

## Use a variable

Pass variables to `println!` with formatting placeholders:

```rust
fn main() {
    let name = "John";
    let age = 30;

    println!("{name} is {age} years old.");
}
```

Output:

```text
John is 30 years old.
```

Formatting arguments are matched with their placeholders. Keep the values and placeholders in the intended order.

## Immutability by default

An ordinary binding cannot be assigned a new value after it is created:

```rust,compile_fail
fn main() {
    let score = 5;
    score = 10; // Error: cannot assign twice to an immutable variable
}
```

This default protects values from changes that were not intended by the programmer.

## Make a variable mutable

Add `mut` when a binding needs to change:

```rust
fn main() {
    let mut score = 5;
    println!("Before: {score}");

    score = 10;
    println!("After: {score}");
}
```

Output:

```text
Before: 5
After: 10
```

Use mutability only when the value must change. The separate `mut` keyword makes that decision visible in the code.

## Add a type annotation

Rust can infer many types, but an annotation makes the intended type explicit:

```rust
fn main() {
    let count: i32 = 30;
    let initial: char = 'J';

    println!("{initial} represents {count}.");
}
```

The type is written after the variable name and a colon.

## Variable scope

A variable is available from the point where it is declared until the end of its scope:

```rust
fn main() {
    let message = "outside";

    {
        let inner_message = "inside";
        println!("{message} and {inner_message}");
    }

    // `inner_message` is no longer available here.
}
```

Inner scopes can use names from their outer scopes. Outer scopes cannot use bindings declared inside an inner scope.

## Quick reference

| Syntax | Meaning |
| --- | --- |
| `let value = 10;` | Create an immutable binding. |
| `let mut value = 10;` | Create a mutable binding. |
| `let value: i32 = 10;` | Create a binding with an explicit type. |
| `println!("{value}");` | Print a variable with captured formatting. |

## Practice

1. Create variables for a name, age, and score.
2. Print them in one sentence.
3. Try changing an immutable variable and read the compiler error.
4. Add `mut` and run the program again.
5. Move one variable into an inner scope and observe where it can be used.

## Source reference

This lesson was informed by the [W3Schools Rust Variables tutorial](https://www.w3schools.com/rust/rust_variables.php) and expanded with Rust-specific guidance on type annotations and scope.
