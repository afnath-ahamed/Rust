# Operators

Operators combine values, compare them, update mutable bindings, or build ranges. Rust requires compatible operand types and does not provide a `++` increment operator.

## Contents

1. [Arithmetic operators](#1-arithmetic-operators)
2. [Assignment operators](#2-assignment-operators)
3. [Comparison operators](#3-comparison-operators)
4. [Logical operators](#4-logical-operators)
5. [Bitwise operators](#5-bitwise-operators)
6. [Range operators](#6-range-operators)
7. [Precedence and grouping](#7-precedence-and-grouping)

## 1. Arithmetic operators

Arithmetic operators work with numeric values:

| Operator | Meaning | Example |
| --- | --- | --- |
| `+` | Addition | `5 + 3` → `8` |
| `-` | Subtraction | `5 - 3` → `2` |
| `*` | Multiplication | `5 * 3` → `15` |
| `/` | Division | `10 / 2` → `5` |
| `%` | Remainder | `10 % 3` → `1` |

```rust
let total = 5 + 3;
let remainder = 10 % 3;
let average = 10.0 / 4.0; // 2.5
```

Integer division discards the fractional part. Do not divide an integer by zero.

## 2. Assignment operators

Assignment requires a mutable binding. Compound assignment updates the existing value:

| Operator | Equivalent operation |
| --- | --- |
| `=` | Assign a value |
| `+=` | `value = value + amount` |
| `-=` | `value = value - amount` |
| `*=` | `value = value * amount` |
| `/=` | `value = value / amount` |
| `%=` | `value = value % amount` |

```rust
let mut score = 10;
score += 5;
score *= 2;
println!("{score}"); // 30
```

## 3. Comparison operators

Comparisons return a `bool`:

| Operator | Meaning |
| --- | --- |
| `==` | Equal to |
| `!=` | Not equal to |
| `>` | Greater than |
| `<` | Less than |
| `>=` | Greater than or equal to |
| `<=` | Less than or equal to |

```rust
let age = 21;
let is_adult = age >= 18;
assert!(is_adult);
```

Both operands must be comparable types. Use `==` for equality; a single `=` assigns a value.

## 4. Logical operators

Logical operators combine boolean expressions and use short-circuit evaluation:

| Operator | Meaning |
| --- | --- |
| `&&` | True when both operands are true |
| `||` | True when at least one operand is true |
| `!` | Inverts a boolean |

```rust
let logged_in = true;
let is_admin = false;
let can_edit = logged_in && is_admin;
let can_view = logged_in || is_admin;
let signed_out = !logged_in;
```

With `a && b`, Rust evaluates `b` only when `a` is true. With `a || b`, it evaluates `b` only when `a` is false.

## 5. Bitwise operators

Bitwise operators work on the individual bits of integer values:

| Operator | Meaning |
| --- | --- |
| `&` | Bitwise AND |
| `|` | Bitwise OR |
| `^` | Bitwise XOR |
| `!` | Bitwise NOT |
| `<<` | Shift left |
| `>>` | Shift right |

```rust
let read = 0b0001u8;
let write = 0b0010u8;
let permissions = read | write;
```

Do not confuse `&` and `|` with the short-circuit boolean operators `&&` and `||`.

## 6. Range operators

Ranges are useful for iteration and slicing:

```rust
for number in 1..=3 {
    println!("{number}");
}

let values = [10, 20, 30, 40];
let first_two = &values[0..2]; // 0 and 1
```

- `start..end` excludes `end`.
- `start..=end` includes `end`.
- `..end`, `start..`, and `..` create open-ended ranges where the context supplies the missing bound.

## 7. Precedence and grouping

Rust evaluates multiplication, division, and remainder before addition and subtraction. Comparisons are evaluated before logical `&&`, which is evaluated before `||`. Use parentheses when intent matters:

```rust
let result = (2 + 3) * 4;
let allowed = is_admin || (logged_in && is_owner);
```

Parentheses improve readability and prevent a change in precedence from changing the meaning of an expression.

## Practice

1. Calculate a total, remainder, and average with arithmetic operators.
2. Update a mutable score using `+=` and `*=`.
3. Compare two values and use the result in an `if` expression.
4. Combine login and permission checks with `&&`, `||`, and `!`.
5. Use a range to print the numbers from 1 through 5.
6. Use bitwise OR to combine two integer flags.

## Source reference

This lesson was informed by the [W3Schools Rust Operators tutorial](https://www.w3schools.com/rust/rust_operators.php) and expanded with Rust-specific guidance on bitwise operators, ranges, short-circuiting, and precedence.

---

[← Previous: Type Conversion](<../07 - Type Conversion/README.md>) | [Back to Programming Fundamentals](../README.md) | [Next: Functions →](<../09 - Functions/README.md>)
