# Type Conversion

Rust does not silently convert one type into another. When values have different types, choose an explicit conversion that communicates whether the operation is always safe, may fail, or changes the value's meaning.

## Contents

1. [Numeric casts with `as`](#1-numeric-casts-with-as)
2. [Parsing text](#2-parsing-text)
3. [Conversions with `From` and `Into`](#3-conversions-with-from-and-into)
4. [Fallible conversions](#4-fallible-conversions)
5. [Strings and values](#5-strings-and-values)
6. [Choosing a conversion](#6-choosing-a-conversion)

## 1. Numeric casts with `as`

Use `as` for primitive numeric casts and conversions between compatible primitive types:

```rust
let count: i32 = 42;
let wide: i64 = count as i64;
let ratio = 3.9f64 as i32; // 3: the fractional part is discarded
let byte = 300u16 as u8;   // wraps according to Rust's cast rules
```

Numeric casts can lose precision, truncate a fraction, or change a value that does not fit. Check the source and destination ranges before using `as` with data that must remain exact.

## 2. Parsing text

Use `str::parse` to convert text into a type that implements `FromStr`. Parsing can fail, so it returns a `Result`:

```rust
let text = "42";
let number: u32 = text.parse().expect("text must contain a positive integer");

let invalid = "not a number".parse::<u32>();
assert!(invalid.is_err());
```

Prefer handling the error with `match`, `?`, or a suitable fallback instead of calling `unwrap` in production code.

## 3. Conversions with `From` and `Into`

`From<T>` defines an infallible conversion into a type. Rust automatically provides the corresponding `Into<T>` implementation:

```rust
let small: u8 = 10;
let number: u32 = u32::from(small);
let also_a_number: u32 = small.into();
```

Use `From` when the destination type is clear. Use `Into` when a function's expected type provides useful context.

## 4. Fallible conversions

When a value may not fit, use `TryFrom` or `TryInto`. These return a `Result` instead of silently changing the value:

```rust
use std::convert::TryFrom;

let value: u16 = 1_000;
let result = u8::try_from(value);

match result {
    Ok(byte) => println!("Fits in a byte: {byte}"),
    Err(error) => println!("Conversion failed: {error}"),
}
```

The equivalent `TryInto` form converts from the source value's perspective:

```rust
use std::convert::TryInto;

let value: u16 = 100;
let byte: Result<u8, _> = value.try_into();
```

Conversions that exceed the destination range return an error instead of silently changing the value:

```rust
let value: i32 = 300;
let result: Result<u8, _> = value.try_into();

assert!(result.is_err()); // 300 exceeds the u8 range
```

Use `?` to propagate a conversion error from a function that returns a compatible `Result`.

## 5. Strings and values

Use `to_string()` or `String::from` to create owned text, and `parse` to convert text back into a typed value:

```rust
let number = 42;
let text = number.to_string();
let owned = String::from("Rust");
let parsed: i32 = text.parse().expect("valid integer");
```

Formatting is useful when combining several values into text:

```rust
let message = format!("Count: {number}");
```

| Method | Purpose |
| --- | --- |
| `as` | Explicit primitive cast that may lose information |
| `parse()` | Convert text into another type and return a `Result` |
| `to_string()` | Convert a value into text |
| `From` / `Into` | Infallible conversion between compatible types |
| `TryFrom` / `TryInto` | Checked conversion that may fail |

## 6. Choosing a conversion

- Use `as` for deliberate primitive numeric casts.
- Use `parse` for text-to-value conversions and handle its `Result`.
- Use `From` or `Into` for conversions guaranteed to succeed.
- Use `TryFrom` or `TryInto` when the value may be out of range.
- Never rely on implicit conversion; Rust requires the conversion to be visible in the code.

## Practice

1. Convert an `i32` into an `i64` with `as`.
2. Parse a user-provided string into a `u32` and handle invalid input.
3. Convert a `u8` into a `u32` with `From`.
4. Try converting a large `u16` into a `u8` with `TryFrom`.
5. Convert a number to text, then parse it back into a number.

## Related lesson

[Error Handling](<../../09 - Error Handling/README.md>) explains how to report and propagate failures from parsing and checked conversions.

---

[← Previous: Type Inference](<../06 - Type Inference/README.md>) | [Back to Programming Fundamentals](../README.md) | [Next: Operators →](<../08 - Operators/README.md>)
