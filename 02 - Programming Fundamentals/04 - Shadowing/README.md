# Shadowing

Shadowing creates a new binding with the same name as an earlier binding. The new binding replaces the old name in the current scope and may have a different value or type.

## Contents

1. [Basic shadowing](#1-basic-shadowing)
2. [Transform a value](#2-transform-a-value)
3. [Change the type](#3-change-the-type)
4. [Shadowing and scope](#4-shadowing-and-scope)
5. [Shadowing versus mutability](#5-shadowing-versus-mutability)
6. [Good practices](#6-good-practices)

## 1. Basic shadowing

Declare a new `let` binding with a name that is already in use:

```rust
fn main() {
    let message = "hello";
    let message = "hello, Rust!";

    println!("{message}");
}
```

Output:

```text
hello, Rust!
```

The second `message` hides the first one from the point where the new binding is declared.

## 2. Transform a value

Shadowing is useful when a value passes through several transformations:

```rust
fn main() {
    let spaces = "   ";
    let spaces = spaces.len();

    println!("The input contains {spaces} spaces.");
}
```

Output:

```text
The input contains 3 spaces.
```

The first binding is a string slice. The second binding stores the resulting length as an integer.

## 3. Change the type

Shadowing can change the type because each `let` statement creates a new binding:

```rust
fn main() {
    let value = "42";
    let value: i32 = value.parse().expect("value should be a number");

    println!("Value: {value}");
}
```

This is different from reassignment through `mut`, which keeps the existing binding's type:

```rust,compile_fail
fn main() {
    let mut value = "42";
    value = 42; // Error: the binding was created as a string slice
}
```

## 4. Shadowing and scope

A shadowed binding follows normal scope rules:

```rust
fn main() {
    let number = 1;

    {
        let number = number + 1;
        println!("Inner value: {number}");
    }

    println!("Outer value: {number}");
}
```

Output:

```text
Inner value: 2
Outer value: 1
```

The inner binding hides the outer binding only inside the inner scope.

## 5. Shadowing versus mutability

| Feature | Shadowing | `mut` reassignment |
| --- | --- | --- |
| Creates a new binding | Yes | No |
| Can change the type | Yes | No |
| Requires another `let` | Yes | No |
| Original binding remains conceptually separate | Yes | No |
| Useful for | Step-by-step transformations | Updating state |

Use shadowing when a value has a new meaning after transformation. Use `mut` when the same value changes during a process.

## 6. Good practices

- Use descriptive names when a transformation becomes difficult to follow.
- Prefer shadowing for short, clear transformations such as parsing or trimming input.
- Use `mut` for counters, accumulators, and state that changes over time.
- Avoid long chains of shadowing that make the active type unclear.
- Remember that shadowing is not mutation and does not change an earlier binding.

## Quick reference

```rust
let value = "42";
let value: i32 = value.parse().unwrap(); // New binding, new type

let mut count = 0;
count += 1; // Same binding, updated value
```

## Practice

1. Shadow a string with its trimmed version.
2. Parse the trimmed string into an integer using another shadowing statement.
3. Rewrite the example with `mut` and observe why the type cannot change.
4. Create an inner scope and compare the inner and outer values with the same name.

---

[← Previous: Constants](<../03 - Constants/README.md>) | [Back to Programming Fundamentals](../README.md) | [Next: Data Types →](<../05 - Data Types/README.md>)
