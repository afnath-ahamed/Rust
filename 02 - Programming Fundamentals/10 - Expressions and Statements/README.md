# Expressions and Statements

Rust is an expression-based language. An **expression** evaluates to a value, while a **statement** performs an action and does not produce a value for the surrounding code.

## Contents

1. [Expressions and statements overview](#1-expressions-and-statements-overview)
2. [Declaration statements](#2-declaration-statements)
3. [Expression statements](#3-expression-statements)
4. [Block expressions](#4-block-expressions)
5. [Literal and path expressions](#5-literal-and-path-expressions)
6. [Operator expressions](#6-operator-expressions)
7. [Function and method calls](#7-function-and-method-calls)
8. [Tuple, array, and indexing expressions](#8-tuple-array-and-indexing-expressions)
9. [Struct and field expressions](#9-struct-and-field-expressions)
10. [Return, break, and continue expressions](#10-return-break-and-continue-expressions)
11. [Closure expressions](#11-closure-expressions)
12. [Expression precedence](#12-expression-precedence)

## 1. Expressions and statements overview

An expression produces a value:

```rust
let total = 2 + 3; // `2 + 3` is an expression
```

A statement performs an action, such as declaring a binding:

```rust
let total = 5; // the complete `let` declaration is a statement
```

Expressions can appear inside statements. A semicolon usually discards an expression's value and turns it into an expression statement.

## 2. Declaration statements

Declaration statements introduce names or items into the current block.

### `let` statements

`let` creates a binding, optionally with a pattern, type annotation, and initializer:

```rust
let count: u32 = 3;
let (width, height) = (800, 600);
```

The binding is visible from its declaration to the end of the enclosing block, unless it is shadowed.

### Item declarations

Functions, structs, enums, and other items can be declared at module level or inside a block. An item declared inside a block is scoped to that block.

## 3. Expression statements

An expression statement evaluates an expression and ignores its result. It commonly ends with a semicolon:

```rust
let mut values = vec![1, 2, 3];
values.push(4); // the return value of `push` is `()`
```

The final expression in a function or block can omit its semicolon when its value should be returned:

```rust
fn answer() -> i32 {
    42
}
```

Adding a semicolon to `42` would make the block return `()` instead of `i32`.

## 4. Block expressions

A block can contain statements and a final expression. The block itself then evaluates to the final value:

```rust
let result = {
    let first = 2;
    let second = 3;
    first + second
};
```

The value of `result` is `5`. Names declared inside the block do not escape it.

## 5. Literal and path expressions

Literal expressions directly represent values such as numbers, characters, strings, booleans, arrays, and tuples:

```rust
let number = 10;
let tuple = ("Rust", 2024);
```

Path expressions refer to names such as local variables, constants, functions, or enum variants:

```rust
const LIMIT: u32 = 10;
let value = LIMIT;
```

## 6. Operator expressions

Operators are expressions that calculate, compare, assign, borrow, dereference, or cast values:

```rust
let total = 2 * (3 + 4);
let is_valid = total > 10;
let reference = &total;
```

See [Operators](<../08 - Operators/README.md>) for the complete operator reference.

## 7. Function and method calls

Calling a function or method is an expression and produces the called operation's return value:

```rust
fn double(value: i32) -> i32 {
    value * 2
}

let result = double(5);
let length = "Rust".len();
```

The `?` operator is also an expression. It returns the successful value or exits the current function with an error or `None`.

## 8. Tuple, array, and indexing expressions

Tuples and arrays are expressions that create compound values:

```rust
let pair = (10, 20);
let values = [10, 20, 30];
let first = values[0];
let second = pair.1;
```

Indexing can panic when a runtime index is out of bounds. Validate indexes when input is not known to be safe.

## 9. Struct and field expressions

Struct expressions create values; field expressions read or update fields:

```rust
struct User {
    name: String,
    active: bool,
}

let user = User {
    name: String::from("Afnath"),
    active: true,
};

println!("{}", user.name);
```

Field access follows ownership and borrowing rules when the field contains an owned value.

See [Structs and Enums](<../../06 - Structs and Enums/README.md>) for data modeling and methods.

## 10. Return, break, and continue expressions

`return` exits a function with a value. `break` exits a loop, optionally with a value, and `continue` skips to the next loop iteration:

```rust
fn find_first_positive(values: &[i32]) -> Option<i32> {
    for &value in values {
        if value > 0 {
            return Some(value);
        }
    }

    None
}
```

Detailed `if`, `loop`, `while`, `for`, `break`, and `continue` behavior belongs in [Control Flow](<../12 - Control Flow/README.md>).

## 11. Closure expressions

Closures are anonymous expressions that can capture values from their surrounding scope:

```rust
let offset = 10;
let add_offset = |value| value + offset;

assert_eq!(add_offset(5), 15);
```

See [Functions](<../09 - Functions/README.md>) for an introduction to closures as reusable behavior.
Continue with [Functional Programming](<../../12 - Functional Programming/README.md>) for capture modes and iterators.

## 12. Expression precedence

Rust evaluates tightly binding expressions before weaker ones. Parentheses make grouping explicit:

```rust
let first = 2 + 3 * 4;     // 14
let second = (2 + 3) * 4;  // 20
```

When in doubt, use parentheses instead of relying on precedence. Assignment and compound assignment bind more weakly than arithmetic and comparisons.

## Practice

1. Write a block expression that calculates a total from two local bindings.
2. Turn a final expression into a statement by adding a semicolon and observe the type error.
3. Create a tuple and array, then access their elements.
4. Define a struct and read one of its fields.
5. Write a closure that captures a local value.
6. Compare two equivalent expressions with and without parentheses.

## Source references

This lesson follows the official [Rust Reference: Expressions](https://doc.rust-lang.org/reference/expressions.html), [Rust Reference: Statements](https://doc.rust-lang.org/reference/statements.html), and the [Rust Book chapter on functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html).

---

[← Previous: Functions](<../09 - Functions/README.md>) | [Back to Programming Fundamentals](../README.md) | [Next: Comments →](<../11 - Comments/README.md>)
