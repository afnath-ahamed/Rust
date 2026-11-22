<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="./assets/rust-logo-white-outline.svg">
    <img src="./assets/rust-logo.svg" alt="Rust programming language logo" width="150">
  </picture>
</div>

<h1 align="center">Rust Learning Diary</h1>

<p align="center">
  A Rust learning diary that starts with the first program and progresses through systems, services, and practical projects.
</p>

## About this repository

This repository organizes my Rust study notes, runnable examples, terminal output, and practical explanations into a numbered learning path. Each section builds on the previous one so the material can be followed from the beginning.

> [!NOTE]
> This is an independent learning diary. It is not an official Rust Project or Rust Foundation resource.

## Learning path

1. [Introduction](<./01 - Introduction/README.md>)
2. [Programming Fundamentals](<./02 - Programming Fundamentals/README.md>)
3. [Memory Management Fundamentals](<./03 - Memory Management Fundamentals/README.md>)
4. [Ownership System](<./04 - Ownership System/README.md>)
5. [Borrowing and References](<./05 - Borrowing and References/README.md>)
6. [Structs and Enums](<./06 - Structs and Enums/README.md>)
7. [Pattern Matching](<./07 - Pattern Matching/README.md>)
8. [Collections and Strings](<./08 - Collections and Strings/README.md>)
9. [Error Handling](<./09 - Error Handling/README.md>)
10. [Project Organization](<./10 - Project Organization/README.md>)
11. [Generics, Traits and Lifetimes](<./11 - Generics, Traits and Lifetimes/README.md>)
12. [Functional Programming](<./12 - Functional Programming/README.md>)
13. [Smart Pointers](<./13 - Smart Pointers/README.md>)
14. [Concurrency and Parallelism](<./14 - Concurrency and Parallelism/README.md>)
15. [Asynchronous Programming](<./15 - Asynchronous Programming/README.md>)
16. [Advanced Rust](<./16 - Advanced Rust/README.md>)
17. [Testing and Debugging](<./17 - Testing and Debugging/README.md>)
18. [Professional Rust Tooling](<./18 - Professional Rust Tooling/README.md>)
19. [Data Structures and Algorithms](<./19 - Data Structures and Algorithms/README.md>)
20. [Backend and Web Development](<./20 - Backend and Web Development/README.md>)
21. [Software Architecture](<./21 - Software Architecture/README.md>)
22. [Performance Engineering](<./22 - Performance Engineering/README.md>)
23. [Security Engineering](<./23 - Security Engineering/README.md>)
24. [Production and DevOps](<./24 - Production and DevOps/README.md>)
25. [Specialization Paths](<./25 - Specialization Paths/README.md>)
26. [Practical Projects](<./26 - Practical Projects/README.md>)

## Repository structure

```text
Rust/
├── 01 - Introduction/
│   ├── 01 - Overview/
│   ├── 02 - Environment Setup/
│   └── 03 - First Rust Program/
├── 02 - Programming Fundamentals/
├── 03 - Memory Management Fundamentals/
├── … (sections 04–25 follow the learning path above)
├── 26 - Practical Projects/
├── assets/
└── README.md
```

## How to use this diary

1. Follow the numbered sections in order.
2. Read each topic's `README.md` for explanations and examples.
3. Run the included Cargo projects or try the examples in a scratch Cargo
   project. Lessons identify when an example belongs in a library or needs
   additional dependencies.
4. Use `cargo check`, `cargo run`, `cargo fmt`, and `cargo clippy` while learning.

## Logo attribution

The Rust logo is © Rust Foundation and is provided by the [Rust Artwork repository](https://github.com/rust-lang/rust-artwork) under the Creative Commons Attribution license. It is used here to refer to the Rust programming language. Rust and Cargo are trademarks of the Rust Foundation. See the [Rust Logo Policy and Media Guide](https://rust-lang.org/policies/media-guide).
