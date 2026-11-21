# Type Inference

Type inference lets Rust determine a value's type from its initializer, surrounding expressions, function signatures, and trait requirements. You usually do not need to write a type annotation, while the compiler still checks that every operation is type-safe.

## Contents

1. [Inference from a value](#1-inference-from-a-value)
2. [Inference from context](#2-inference-from-context)
3. [When an annotation is useful](#3-when-an-annotation-is-useful)
4. [Ambiguous numeric types](#4-ambiguous-numeric-types)
5. [Inference is not conversion](#5-inference-is-not-conversion)
6. [Best practices](#6-best-practices)

## 1. Inference from a value

Rust examines the initializer and infers a suitable type:

```rust
let count = 10;          // i32 by default
let price = 19.99;       // f64 by default
let enabled = true;      // bool
let initial = 'R';       // char
let greeting = "Hello";  // &str
```

Inference happens at compile time. It does not make Rust dynamically typed, and the type cannot change later just because another value is assigned.

## 2. Inference from context

The compiler combines information from surrounding code. A function parameter, return type, method, or expected collection element type can determine an otherwise unspecified type:

```rust
fn double(value: i64) -> i64 {
    value * 2
}

let result = double(21); // the argument must be i64
```

Generic collections also provide context:

```rust
let values: Vec<u32> = vec![10, 20, 30];
let total: u32 = values.iter().sum();
```

## 3. When an annotation is useful

Add an annotation when the intended type is important to the reader, when an API requires it, or when the compiler cannot determine a unique type:

```rust
let limit: usize = 100;
let port: u16 = 8080;
let names: Vec<String> = Vec::new();
```

Annotations document intent and can produce clearer compiler errors. They do not perform a conversion; the assigned expression must already have the declared type.

## 4. Ambiguous numeric types

Some literals can represent several numeric types. Rust uses defaults when there is enough information, but methods such as `parse` often need an annotation:

```rust
let count = "42".parse::<u32>().expect("valid number");
let distance: i64 = "120".parse().expect("valid distance");
let masked = 255u8;
```

If the compiler reports that it cannot infer a type, annotate the binding, the expression, or the generic type parameter.

## 5. Inference is not conversion

Inference chooses a type for an expression; it does not change the value's type. Rust will not silently combine incompatible numeric types:

```rust
let whole: i32 = 10;
let decimal: f64 = 2.5;

// let total = whole + decimal; // Error: different numeric types
let total = f64::from(whole) + decimal;
```

See [Type Conversion](<../07 - Type Conversion/README.md>) for casts, parsing, and `From`/`Into` conversions.

## 6. Best practices

- Let inference handle obvious local values.
- Add annotations at public API boundaries and where they communicate intent.
- Prefer explicit annotations over unclear chains of generic calls.
- Read compiler diagnostics instead of guessing when inference fails.
- Use type suffixes such as `10u64` for concise numeric literals.

## Practice

1. Predict the inferred type of several integer, float, character, and string literals.
2. Pass an inferred value to a function with a typed parameter.
3. Create an empty `Vec` and give it an element type using context.
4. Parse a string into both `u32` and `f64` values.
5. Trigger an inference error, then fix it with an annotation.

---

[← Previous: Data Types](<../05 - Data Types/README.md>) | [Back to Language Basics](../README.md) | [Next: Type Conversion →](<../07 - Type Conversion/README.md>)
