# 11 — Comments

Comments explain code for people. Rust ignores ordinary comments when compiling the program, so they do not change the program's runtime behavior.

## Contents

1. [Line comments](#line-comments)
2. [Block comments](#block-comments)
3. [Documentation comments](#documentation-comments)
4. [Practical guidance](#practical-guidance)

## Line comments

Start a line comment with `//`. Everything after `//` until the end of the line is ignored.

```rust
// Print a greeting to the terminal.
println!("Hello, Rust!");

println!("Hello, Rust!"); // An end-of-line comment
```

Use line comments for short explanations close to the code they describe.

## Block comments

Start a block comment with `/*` and end it with `*/`. Block comments can span multiple lines. Rust also supports nested block comments.

```rust
/*
   This comment explains a larger section
   without placing `//` on every line.
*/
let answer = 6 * 7;
```

Block comments are useful for temporarily describing or disabling a larger section while experimenting.

## Documentation comments

Documentation comments describe public items and are processed by `rustdoc`.

### Item documentation with `///`

Use `///` immediately before a function, struct, enum, or other item.

```rust
/// Adds two numbers and returns the result.
fn add(left: i32, right: i32) -> i32 {
    left + right
}
```

### Module or crate documentation with `//!`

Use `//!` at the beginning of a module or crate to document the module itself.

```rust
//! Utilities for basic arithmetic.

/// Returns the square of a number.
pub fn square(value: i32) -> i32 {
    value * value
}
```

Markdown inside documentation comments is rendered by `rustdoc`. You can generate documentation with:

```powershell
cargo doc --open
```

## Practical guidance

- Explain **why** a non-obvious decision exists, not what an immediately clear line does.
- Keep comments close to the code they describe.
- Update comments when the implementation changes.
- Use `///` for public API documentation.
- Prefer clear names and simple code over comments that repeat the code.
- Remove temporary debugging comments before committing.

## Quick reference

| Syntax | Purpose |
| --- | --- |
| `// text` | Single-line comment |
| `/* text */` | Multi-line block comment |
| `/// text` | Documentation for the following item |
| `//! text` | Documentation for the current module or crate |

## Source reference

This lesson was informed by the [W3Schools Rust Comments tutorial](https://www.w3schools.com/rust/rust_comments.php) and expanded with Rust documentation-comment and `rustdoc` guidance.
