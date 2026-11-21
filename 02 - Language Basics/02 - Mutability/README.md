# Mutability

Rust variables are immutable by default. A binding can be changed only when it is declared with `mut`, making intended changes visible in the source code.

## Contents

1. [Immutable bindings](#1-immutable-bindings)
2. [Mutable bindings](#2-mutable-bindings)
3. [What `mut` changes](#3-what-mut-changes)
4. [Mutability and scope](#4-mutability-and-scope)
5. [Mutability versus shadowing](#5-mutability-versus-shadowing)
6. [Good practices](#6-good-practices)

## 1. Immutable bindings

Use `let` to create an immutable binding:

```rust
fn main() {
    let temperature = 20;
    println!("Temperature: {temperature}°C");
}
```

Trying to assign a new value produces a compiler error:

```rust,compile_fail
fn main() {
    let temperature = 20;
    temperature = 25;
}
```

The binding is not allowed to change after its initial assignment.

## 2. Mutable bindings

Add `mut` when the value must be reassigned:

```rust
fn main() {
    let mut temperature = 20;
    println!("Before: {temperature}°C");

    temperature = 25;
    println!("After: {temperature}°C");
}
```

Output:

```text
Before: 20°C
After: 25°C
```

The `mut` keyword applies to the binding. It permits assigning a new value of the same type to that binding.

## 3. What `mut` changes

`mut` allows reassignment, but it does not remove Rust's other safety checks:

- The new value must have a compatible type.
- The binding must still be in scope.
- Borrowing rules still apply when the value is referenced.
- `mut` does not make a value safe to share between threads automatically.

```rust,compile_fail
fn main() {
    let mut count = 1;
    count = "two"; // Error: expected an integer, found a string slice
}
```

## 4. Mutability and scope

Mutability belongs to a particular binding and does not automatically apply to another binding:

```rust
fn main() {
    let mut count = 1;

    {
        count += 1;
        println!("Inner scope: {count}");
    }

    count += 1;
    println!("Outer scope: {count}");
}
```

The mutable binding remains usable while its enclosing scope is active. A binding declared inside an inner scope is unavailable outside that scope.

## 5. Mutability versus shadowing

Mutability changes the existing binding. Shadowing creates a new binding with the same name:

```rust
fn main() {
    let spaces = "   ";
    let spaces = spaces.len();

    println!("Number of spaces: {spaces}");
}
```

Shadowing can change the type. Reassignment through `mut` cannot change the type of the existing binding.

## 6. Good practices

- Start with immutable bindings and add `mut` only when reassignment is required.
- Keep mutable state close to the code that updates it.
- Prefer a new binding or a function when a transformation represents a new concept.
- Read compiler errors carefully when a change violates type or borrowing rules.
- Use meaningful names so the reason for changing a value is clear.

## 7. Quick reference

| Syntax | Meaning |
| --- | --- |
| `let value = 10;` | Immutable binding. |
| `let mut value = 10;` | Mutable binding that can be reassigned. |
| `value = 20;` | Reassign an existing mutable binding. |
| `let value = value + 1;` | Shadow the previous binding with a new one. |

## 8. Practice

1. Create an immutable counter and try to increment it.
2. Add `mut` and run the program again.
3. Try assigning a value of a different type and read the compiler error.
4. Rewrite the example using shadowing instead of reassignment.

---

[← Previous: Variables](<../01 - Variables/README.md>) | [Back to Language Basics](../README.md) | [Next: Constants →](<../03 - Constants/README.md>)
