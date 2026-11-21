# Your First Rust Program

This lesson introduces the structure of a Cargo project and the basic parts of a Rust program. The included project prints a message to the terminal and can be run immediately.

## Contents

1. [Create the project](#create-the-project)
2. [Project structure](#project-structure)
3. [Program source](#program-source)
4. [How the program works](#how-the-program-works)
5. [Run the program](#run-the-program)
6. [Useful development commands](#useful-development-commands)

## Create the project

Open PowerShell in the folder where you keep your Rust projects and run:

```powershell
cargo new hello_rust
cd hello_rust
```

Cargo creates a new binary project with a `Cargo.toml` manifest and a `src/main.rs` entry point. Replace the generated `main.rs` content with the example in this lesson, then run:

```powershell
cargo run
```

To create a project in a specific location, provide a path instead:

```powershell
cargo new "D:\Projects\hello_rust"
cd "D:\Projects\hello_rust"
```

Use a valid, unique project name with lowercase letters, numbers, underscores, or hyphens. Cargo uses the project name as the default package name.

## Project structure

```text
hello_rust/
├── .gitignore
├── Cargo.lock
├── Cargo.toml
└── src/
    └── main.rs
```

- `.gitignore` keeps generated build artifacts such as `target/` out of version control.
- `Cargo.lock` records the exact dependency versions selected for the build.
- `Cargo.toml` contains the package name, version, Rust edition, and dependencies.
- `src/main.rs` is the entry point for this executable project.

The lesson README is documentation stored beside the project. It is not required for the Cargo build itself.

## Program source

```rust
fn main() {
    println!("Hello, Rust!");
}
```

## How the program works

1. **`fn main()`**
   - Defines the program's entry point. Execution begins inside this function.

2. **`println!`**
   - Prints text followed by a new line.
   - The `!` indicates that `println!` is a macro.

3. **`"Hello, Rust!"`**
   - A string literal containing the text displayed in the terminal.

4. **Semicolon (`;`)**
   - Ends the `println!` statement.

## Run the program

Open PowerShell in this folder and run:

```powershell
cargo run
```

Expected program output:

```text
Hello, Rust!
```

## Useful development commands

| Command | Purpose |
| --- | --- |
| `cargo check` | Check the code for errors without producing the final executable. |
| `cargo run` | Build and run the program. |
| `cargo build` | Create a debug build under `target/debug/`. |
| `cargo build --release` | Create an optimized build under `target/release/`. |
| `cargo fmt` | Format the source code. |
| `cargo clippy` | Check the code and suggest improvements. |

## Next step

Continue with [Rust Language Basics](<../../02 - Language Basics/README.md>) to learn variables, data types, functions, and control flow.
