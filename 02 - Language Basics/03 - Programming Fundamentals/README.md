# 03 — Programming Fundamentals

This section covers the essential Rust language features used in everyday programs. Study the topics in order, run the examples, and change the code to test your understanding.

## Contents

1. [Variables](#variables)
2. [Mutability](#mutability)
3. [Constants](#constants)
4. [Shadowing](#shadowing)
5. [Data types](#data-types)
6. [Type inference](#type-inference)
7. [Type conversion](#type-conversion)
8. [Operators](#operators)
9. [Functions](#functions)
10. [Expressions and statements](#expressions-and-statements)
11. [Comments](#comments)
12. [Control flow](#control-flow)

## Variables

Learn how `let` binds a name to a value and how Rust keeps bindings immutable by default.

## Mutability

Use `mut` when a binding must change, and understand why explicit mutability makes changes easier to track.

## Constants

Define values that never change with `const`, including the required type annotation and naming convention.

## Shadowing

Reuse a name with a new `let` binding to transform a value or change its type without making the original binding mutable.

## Data types

### Scalar types

Study integers, floating-point numbers, booleans, and characters.

### Compound types

Work with tuples and arrays, including indexing, destructuring, and fixed-size array behavior.

## Type inference

See how the compiler determines many types from how values are used, and learn when an explicit annotation improves clarity or resolves ambiguity.

## Type conversion

Convert values explicitly with methods such as `as`, `parse`, and `from` or `into` where appropriate. Rust does not silently convert unrelated numeric types.

## Operators

Use arithmetic, comparison, logical, assignment, range, and remainder operators to build expressions.

## Functions

Define reusable functions with parameters and return types. Learn how the final expression in a function can return a value.

## Expressions and statements

Distinguish expressions, which evaluate to values, from statements, which perform an action or declaration.

## Comments

Write line comments, block comments, and documentation comments that explain intent and public interfaces.

## Control flow

### If / Else

Choose between branches with boolean conditions and use `else if` for multiple cases.

### Loop

Repeat a block with `loop`, exit with `break`, and return a value from a loop when needed.

### While

Repeat while a condition remains true.

### For

Iterate safely over ranges and collections with `for`.

## Study checklist

- [ ] Declare immutable and mutable variables.
- [ ] Define constants and shadow an existing binding.
- [ ] Identify scalar and compound types.
- [ ] Add type annotations and perform explicit conversions.
- [ ] Use operators in expressions.
- [ ] Write functions with parameters and return values.
- [ ] Explain the difference between an expression and a statement.
- [ ] Add clear comments and documentation comments.
- [ ] Build examples with `if`, `loop`, `while`, and `for`.
