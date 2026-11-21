# Your First Rust Program

This lesson introduces the structure of a Cargo project and the basic parts of a Rust program. The included project prints a message to the terminal and can be run immediately.

## Contents

1. [Project structure](#project-structure)
2. [Program source](#program-source)
3. [How the program works](#how-the-program-works)
4. [Run the program](#run-the-program)
5. [Useful development commands](#useful-development-commands)

## Project structure

```text
03 - First Rust Program/
├── Cargo.toml
├── README.md
└── src/
    └── main.rs
```

- `Cargo.toml` contains the package name, version, Rust edition, and dependencies.
- `src/main.rs` is the entry point for this executable project.
- `README.md` explains the lesson and commands.

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
