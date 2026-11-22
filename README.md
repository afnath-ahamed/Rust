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

   See where Rust is used and how its compiler and ecosystem fit together. Set up Rustup, Cargo, and VS Code on Windows, then create and run a first Cargo project.

2. [Programming Fundamentals](<./02 - Programming Fundamentals/README.md>)

   Write small programs with variables, mutability, constants, and shadowing. Learn scalar and compound types, inference, conversions, operators, functions, expressions, comments, and control flow.

3. [Memory Management Fundamentals](<./03 - Memory Management Fundamentals/README.md>)

   Follow how stack frames and heap allocations are created and released. Compare Rust’s cleanup model with garbage collection and learn how dangling pointers and memory leaks differ.

4. [Ownership System](<./04 - Ownership System/README.md>)

   Trace who owns a value across scopes, assignments, function calls, and returns. Distinguish moves, implicit copies, and explicit clones, and see when `Drop` runs.

5. [Borrowing and References](<./05 - Borrowing and References/README.md>)

   Use shared and mutable references to work with existing values. Learn how the borrow checker prevents dangling references, how dereferencing works, and when to accept a slice.

6. [Structs and Enums](<./06 - Structs and Enums/README.md>)

   Model related fields and alternative states with structs and enums. Define methods and associated functions in `impl` blocks, and use `Option` and `Result` to represent outcomes.

7. [Pattern Matching](<./07 - Pattern Matching/README.md>)

   Extract values with destructuring and choose between `match`, `if let`, `while let`, and `let ... else`. Add guards and handle all variants explicitly when each one matters.

8. [Collections and Strings](<./08 - Collections and Strings/README.md>)

   Choose among arrays, tuples, vectors, maps, sets, and queues by how you access and update data. Use owned `String` and borrowed `&str`, and handle UTF-8 boundaries correctly.

9. [Error Handling](<./09 - Error Handling/README.md>)

   Distinguish recoverable failures from panics. Use `Result`, `Option`, and `?`, understand when `unwrap` or `expect` can panic, and define useful errors with `thiserror` or `anyhow`.

10. [Project Organization](<./10 - Project Organization/README.md>)

    Organize code into packages, library and binary crates, modules, and Cargo workspaces. Control visibility and imports, and manage dependencies, features, and reproducible builds with `Cargo.toml` and `Cargo.lock`.

11. [Generics, Traits and Lifetimes](<./11 - Generics, Traits and Lifetimes/README.md>)

    Write reusable functions and types with generics, trait bounds, default methods, and associated types. Express borrowing relationships through lifetimes and compare static dispatch with trait objects.

12. [Functional Programming](<./12 - Functional Programming/README.md>)

    Capture state in closures and choose among `Fn`, `FnMut`, and `FnOnce`. Build lazy iterator pipelines with `map`, `filter`, `fold`, and `collect`, and recognize when intermediate allocations add work.

13. [Smart Pointers](<./13 - Smart Pointers/README.md>)

    Use `Box`, `Rc`, `Arc`, and `Cow` for different ownership needs. Learn interior mutability with `Cell` and `RefCell`, how `Deref` and `Drop` work, and how `Weak` can prevent reference cycles.

14. [Concurrency and Parallelism](<./14 - Concurrency and Parallelism/README.md>)

    Distinguish concurrency from parallelism and coordinate threads with channels, locks, and atomics. Use `Send` and `Sync` to reason about sharing, recognize races and deadlocks, and try Rayon for data parallelism.

15. [Asynchronous Programming](<./15 - Asynchronous Programming/README.md>)

    Follow a future from creation to completion under an executor. Use `async` and `.await` with Tokio, tasks, channels, and streams, and reason about blocking, cancellation, pinning, and errors.

16. [Advanced Rust](<./16 - Advanced Rust/README.md>)

    Document safety conditions for raw pointers, unsafe functions, and foreign calls. Explore advanced traits, const generics, type-state, higher-ranked bounds, generic associated types, macros, type aliases, and `no_std` libraries.

17. [Testing and Debugging](<./17 - Testing and Debugging/README.md>)

    Check behavior with unit, integration, documentation, property-based, and fuzz tests. Use fakes, compiler diagnostics, backtraces, logs, traces, coverage, and benchmarks to investigate failures and performance.

18. [Professional Rust Tooling](<./18 - Professional Rust Tooling/README.md>)

    Use Cargo, Rustfmt, Clippy, and Rustdoc in daily work. Check dependencies with Cargo Audit and Cargo Deny, investigate unsafe code with Miri, and work with build profiles, cross-compilation, and publishing.

19. [Data Structures and Algorithms](<./19 - Data Structures and Algorithms/README.md>)

    Compare algorithms by time and space cost. Practice searching, sorting, recursion, and dynamic programming, and choose among linked lists, stacks, queues, trees, and graphs.

20. [Backend and Web Development](<./20 - Backend and Web Development/README.md>)

    Build HTTP and REST services with Axum or Actix Web. Connect routing, middleware, Serde, PostgreSQL, SQLx or Diesel, authentication and authorization, WebSockets, gRPC, caching, and background jobs.

21. [Software Architecture](<./21 - Software Architecture/README.md>)

    Design boundaries with SOLID principles, design patterns, dependency injection, and domain-driven design. Compare layered, clean, and hexagonal approaches, then weigh modular monoliths, microservices, and event-driven systems.

22. [Performance Engineering](<./22 - Performance Engineering/README.md>)

    Measure workloads with benchmarks, profiles, and flame graphs. Use the results to guide changes to CPU work, memory, allocations, cache locality, SIMD, release settings, and binary size.

23. [Security Engineering](<./23 - Security Engineering/README.md>)

    Apply memory safety, input validation, and secure coding at trust boundaries. Learn cryptography fundamentals, manage secrets, review dependencies and build provenance, and audit unsafe code.

24. [Production and DevOps](<./24 - Production and DevOps/README.md>)

    Configure, containerize, and release Rust services with Docker, CI/CD, GitHub Actions, and Kubernetes. Plan cloud deployment, health checks, graceful shutdown, observability, monitoring, and rollback.

25. [Specialization Paths](<./25 - Specialization Paths/README.md>)

    Pick a direction through small projects in backend, systems, networking, embedded, operating systems, CLI, Tauri, WebAssembly, blockchain, FinTech, games, cybersecurity, or AI/ML infrastructure.

26. [Practical Projects](<./26 - Practical Projects/README.md>)

    Apply the roadmap through beginner, intermediate, advanced, and professional builds. Start with the runnable calculator and temperature converter, then work through guided projects involving files, databases, concurrency, distributed services, and open-source contribution.

## Repository structure

```text
Rust/
├── 01 - Introduction/
├── 02 - Programming Fundamentals/
├── 03 - Memory Management Fundamentals/
├── 04 - Ownership System/
├── 05 - Borrowing and References/
├── 06 - Structs and Enums/
├── 07 - Pattern Matching/
├── 08 - Collections and Strings/
├── 09 - Error Handling/
├── 10 - Project Organization/
├── 11 - Generics, Traits and Lifetimes/
├── 12 - Functional Programming/
├── 13 - Smart Pointers/
├── 14 - Concurrency and Parallelism/
├── 15 - Asynchronous Programming/
├── 16 - Advanced Rust/
├── 17 - Testing and Debugging/
├── 18 - Professional Rust Tooling/
├── 19 - Data Structures and Algorithms/
├── 20 - Backend and Web Development/
├── 21 - Software Architecture/
├── 22 - Performance Engineering/
├── 23 - Security Engineering/
├── 24 - Production and DevOps/
├── 25 - Specialization Paths/
├── 26 - Practical Projects/
├── assets/
└── README.md
```

<details>
<summary>Expand the full repository tree</summary>

```text
Rust/
├── 01 - Introduction/
│   ├── 01 - Overview/
│   │   └── README.md
│   ├── 02 - Environment Setup/
│   │   ├── README.md
│   │   ├── rust-analyzer-extension-card.png
│   │   └── visual-studio-build-tools.png
│   ├── 03 - First Rust Program/
│   │   ├── src/
│   │   │   └── main.rs
│   │   ├── .gitignore
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   └── README.md
│   └── README.md
├── 02 - Programming Fundamentals/
│   ├── 01 - Variables/
│   │   └── README.md
│   ├── 02 - Mutability/
│   │   └── README.md
│   ├── 03 - Constants/
│   │   └── README.md
│   ├── 04 - Shadowing/
│   │   └── README.md
│   ├── 05 - Data Types/
│   │   └── README.md
│   ├── 06 - Type Inference/
│   │   └── README.md
│   ├── 07 - Type Conversion/
│   │   └── README.md
│   ├── 08 - Operators/
│   │   └── README.md
│   ├── 09 - Functions/
│   │   └── README.md
│   ├── 10 - Expressions and Statements/
│   │   └── README.md
│   ├── 11 - Comments/
│   │   └── README.md
│   ├── 12 - Control Flow/
│   │   └── README.md
│   └── README.md
├── 03 - Memory Management Fundamentals/
│   ├── 01 - Stack memory/
│   │   └── README.md
│   ├── 02 - Heap memory/
│   │   └── README.md
│   ├── 03 - Memory allocation/
│   │   └── README.md
│   ├── 04 - Memory deallocation/
│   │   └── README.md
│   ├── 05 - Pointers/
│   │   └── README.md
│   ├── 06 - Garbage collection/
│   │   └── README.md
│   ├── 07 - Dangling pointers/
│   │   └── README.md
│   ├── 08 - Memory leaks/
│   │   └── README.md
│   ├── 09 - Memory safety/
│   │   └── README.md
│   └── README.md
├── 04 - Ownership System/
│   ├── 01 - Ownership rules/
│   │   └── README.md
│   ├── 02 - Variable scope/
│   │   └── README.md
│   ├── 03 - Move semantics/
│   │   └── README.md
│   ├── 04 - Copy semantics/
│   │   └── README.md
│   ├── 05 - Clone/
│   │   └── README.md
│   ├── 06 - Ownership with functions/
│   │   └── README.md
│   ├── 07 - Return values and ownership/
│   │   └── README.md
│   ├── 08 - Drop/
│   │   └── README.md
│   └── README.md
├── 05 - Borrowing and References/
│   ├── 01 - References/
│   │   └── README.md
│   ├── 02 - Immutable borrowing/
│   │   └── README.md
│   ├── 03 - Mutable borrowing/
│   │   └── README.md
│   ├── 04 - Borrowing rules/
│   │   └── README.md
│   ├── 05 - Borrow checker/
│   │   └── README.md
│   ├── 06 - Dangling references/
│   │   └── README.md
│   ├── 07 - Dereferencing/
│   │   └── README.md
│   ├── 08 - Slices/
│   │   └── README.md
│   └── README.md
├── 06 - Structs and Enums/
│   ├── 01 - Structs/
│   │   └── README.md
│   ├── 02 - Tuple structs/
│   │   └── README.md
│   ├── 03 - Unit-like structs/
│   │   └── README.md
│   ├── 04 - Methods/
│   │   └── README.md
│   ├── 05 - Associated functions/
│   │   └── README.md
│   ├── 06 - Implementation blocks (impl)/
│   │   └── README.md
│   ├── 07 - Enums/
│   │   └── README.md
│   ├── 08 - Option/
│   │   └── README.md
│   ├── 09 - Result/
│   │   └── README.md
│   └── README.md
├── 07 - Pattern Matching/
│   ├── 01 - Match expressions/
│   │   └── README.md
│   ├── 02 - If let/
│   │   └── README.md
│   ├── 03 - While let/
│   │   └── README.md
│   ├── 04 - Let else/
│   │   └── README.md
│   ├── 05 - Destructuring/
│   │   └── README.md
│   ├── 06 - Match guards/
│   │   └── README.md
│   ├── 07 - Exhaustive matching/
│   │   └── README.md
│   └── README.md
├── 08 - Collections and Strings/
│   ├── 01 - Arrays/
│   │   └── README.md
│   ├── 02 - Tuples/
│   │   └── README.md
│   ├── 03 - Vectors/
│   │   └── README.md
│   ├── 04 - String vs Str/
│   │   └── README.md
│   ├── 05 - HashMap/
│   │   └── README.md
│   ├── 06 - HashSet/
│   │   └── README.md
│   ├── 07 - BTreeMap/
│   │   └── README.md
│   ├── 08 - VecDeque/
│   │   └── README.md
│   ├── 09 - UTF-8 encoding/
│   │   └── README.md
│   └── README.md
├── 09 - Error Handling/
│   ├── 01 - Recoverable errors/
│   │   └── README.md
│   ├── 02 - Unrecoverable errors/
│   │   └── README.md
│   ├── 03 - Panic/
│   │   └── README.md
│   ├── 04 - Result/
│   │   └── README.md
│   ├── 05 - Option/
│   │   └── README.md
│   ├── 06 - Unwrap and Expect/
│   │   └── README.md
│   ├── 07 - Error propagation/
│   │   └── README.md
│   ├── 08 - Custom errors/
│   │   └── README.md
│   ├── 09 - Thiserror/
│   │   └── README.md
│   ├── 10 - Anyhow/
│   │   └── README.md
│   └── README.md
├── 10 - Project Organization/
│   ├── 01 - Packages/
│   │   └── README.md
│   ├── 02 - Crates/
│   │   └── README.md
│   ├── 03 - Modules/
│   │   └── README.md
│   ├── 04 - Module tree/
│   │   └── README.md
│   ├── 05 - Public and private visibility/
│   │   └── README.md
│   ├── 06 - Use keyword/
│   │   └── README.md
│   ├── 07 - Library crates/
│   │   └── README.md
│   ├── 08 - Binary crates/
│   │   └── README.md
│   ├── 09 - Cargo.toml/
│   │   └── README.md
│   ├── 10 - Cargo.lock/
│   │   └── README.md
│   ├── 11 - Dependencies/
│   │   └── README.md
│   ├── 12 - Features/
│   │   └── README.md
│   ├── 13 - Cargo workspaces/
│   │   └── README.md
│   └── README.md
├── 11 - Generics, Traits and Lifetimes/
│   ├── 01 - Generic functions/
│   │   └── README.md
│   ├── 02 - Generic structs/
│   │   └── README.md
│   ├── 03 - Generic enums/
│   │   └── README.md
│   ├── 04 - Traits/
│   │   └── README.md
│   ├── 05 - Trait implementation/
│   │   └── README.md
│   ├── 06 - Trait bounds/
│   │   └── README.md
│   ├── 07 - Default implementations/
│   │   └── README.md
│   ├── 08 - Associated types/
│   │   └── README.md
│   ├── 09 - Lifetime annotations/
│   │   └── README.md
│   ├── 10 - Lifetime elision/
│   │   └── README.md
│   ├── 11 - Static lifetime/
│   │   └── README.md
│   ├── 12 - Trait objects/
│   │   └── README.md
│   ├── 13 - Static vs Dynamic dispatch/
│   │   └── README.md
│   └── README.md
├── 12 - Functional Programming/
│   ├── 01 - Closures/
│   │   └── README.md
│   ├── 02 - Fn, FnMut and FnOnce/
│   │   └── README.md
│   ├── 03 - Iterators/
│   │   └── README.md
│   ├── 04 - Iterator adapters/
│   │   └── README.md
│   ├── 05 - Map/
│   │   └── README.md
│   ├── 06 - Filter/
│   │   └── README.md
│   ├── 07 - Fold/
│   │   └── README.md
│   ├── 08 - Collect/
│   │   └── README.md
│   ├── 09 - Lazy evaluation/
│   │   └── README.md
│   ├── 10 - Zero-cost abstractions/
│   │   └── README.md
│   └── README.md
├── 13 - Smart Pointers/
│   ├── 01 - Box/
│   │   └── README.md
│   ├── 02 - Rc/
│   │   └── README.md
│   ├── 03 - Arc/
│   │   └── README.md
│   ├── 04 - RefCell/
│   │   └── README.md
│   ├── 05 - Cell/
│   │   └── README.md
│   ├── 06 - Weak/
│   │   └── README.md
│   ├── 07 - Cow/
│   │   └── README.md
│   ├── 08 - Deref trait/
│   │   └── README.md
│   ├── 09 - Drop trait/
│   │   └── README.md
│   ├── 10 - Interior mutability/
│   │   └── README.md
│   ├── 11 - Reference cycles/
│   │   └── README.md
│   └── README.md
├── 14 - Concurrency and Parallelism/
│   ├── 01 - Processes vs Threads/
│   │   └── README.md
│   ├── 02 - Multithreading/
│   │   └── README.md
│   ├── 03 - Thread spawning/
│   │   └── README.md
│   ├── 04 - Move closures/
│   │   └── README.md
│   ├── 05 - Message passing/
│   │   └── README.md
│   ├── 06 - Channels/
│   │   └── README.md
│   ├── 07 - Shared state/
│   │   └── README.md
│   ├── 08 - Mutex/
│   │   └── README.md
│   ├── 09 - RwLock/
│   │   └── README.md
│   ├── 10 - Send and Sync traits/
│   │   └── README.md
│   ├── 11 - Atomics/
│   │   └── README.md
│   ├── 12 - Race conditions/
│   │   └── README.md
│   ├── 13 - Deadlocks/
│   │   └── README.md
│   ├── 14 - Rayon/
│   │   └── README.md
│   └── README.md
├── 15 - Asynchronous Programming/
│   ├── 01 - Synchronous vs Asynchronous/
│   │   └── README.md
│   ├── 02 - Blocking vs Non-blocking/
│   │   └── README.md
│   ├── 03 - Async and Await/
│   │   └── README.md
│   ├── 04 - Futures/
│   │   └── README.md
│   ├── 05 - Executors/
│   │   └── README.md
│   ├── 06 - Async runtimes/
│   │   └── README.md
│   ├── 07 - Tokio/
│   │   └── README.md
│   ├── 08 - Async tasks/
│   │   └── README.md
│   ├── 09 - Async channels/
│   │   └── README.md
│   ├── 10 - Streams/
│   │   └── README.md
│   ├── 11 - Cancellation/
│   │   └── README.md
│   ├── 12 - Pin and Unpin/
│   │   └── README.md
│   ├── 13 - Async error handling/
│   │   └── README.md
│   └── README.md
├── 16 - Advanced Rust/
│   ├── 01 - Unsafe Rust/
│   │   └── README.md
│   ├── 02 - Raw pointers/
│   │   └── README.md
│   ├── 03 - Unsafe functions/
│   │   └── README.md
│   ├── 04 - Advanced traits/
│   │   └── README.md
│   ├── 05 - Associated constants/
│   │   └── README.md
│   ├── 06 - Const generics/
│   │   └── README.md
│   ├── 07 - Type aliases/
│   │   └── README.md
│   ├── 08 - Type-state pattern/
│   │   └── README.md
│   ├── 09 - Higher-ranked trait bounds/
│   │   └── README.md
│   ├── 10 - Generic associated types/
│   │   └── README.md
│   ├── 11 - Declarative macros/
│   │   └── README.md
│   ├── 12 - Procedural macros/
│   │   └── README.md
│   ├── 13 - Foreign Function Interface (FFI)/
│   │   └── README.md
│   ├── 14 - no_std Rust/
│   │   └── README.md
│   └── README.md
├── 17 - Testing and Debugging/
│   ├── 01 - Unit testing/
│   │   └── README.md
│   ├── 02 - Integration testing/
│   │   └── README.md
│   ├── 03 - Documentation testing/
│   │   └── README.md
│   ├── 04 - Property-based testing/
│   │   └── README.md
│   ├── 05 - Mocking/
│   │   └── README.md
│   ├── 06 - Fuzz testing/
│   │   └── README.md
│   ├── 07 - Benchmarking/
│   │   └── README.md
│   ├── 08 - Debugging/
│   │   └── README.md
│   ├── 09 - Logging/
│   │   └── README.md
│   ├── 10 - Tracing/
│   │   └── README.md
│   ├── 11 - Code coverage/
│   │   └── README.md
│   └── README.md
├── 18 - Professional Rust Tooling/
│   ├── 01 - Cargo commands/
│   │   └── README.md
│   ├── 02 - Rustfmt/
│   │   └── README.md
│   ├── 03 - Clippy/
│   │   └── README.md
│   ├── 04 - Rustdoc/
│   │   └── README.md
│   ├── 05 - Cargo Audit/
│   │   └── README.md
│   ├── 06 - Cargo Deny/
│   │   └── README.md
│   ├── 07 - Miri/
│   │   └── README.md
│   ├── 08 - Compiler diagnostics/
│   │   └── README.md
│   ├── 09 - Build profiles/
│   │   └── README.md
│   ├── 10 - Cross-compilation/
│   │   └── README.md
│   ├── 11 - Publishing crates/
│   │   └── README.md
│   └── README.md
├── 19 - Data Structures and Algorithms/
│   ├── 01 - Time and space complexity/
│   │   └── README.md
│   ├── 02 - Searching algorithms/
│   │   └── README.md
│   ├── 03 - Sorting algorithms/
│   │   └── README.md
│   ├── 04 - Linked lists/
│   │   └── README.md
│   ├── 05 - Stacks/
│   │   └── README.md
│   ├── 06 - Queues/
│   │   └── README.md
│   ├── 07 - Trees/
│   │   └── README.md
│   ├── 08 - Graphs/
│   │   └── README.md
│   ├── 09 - Recursion/
│   │   └── README.md
│   ├── 10 - Dynamic programming/
│   │   └── README.md
│   └── README.md
├── 20 - Backend and Web Development/
│   ├── 01 - HTTP fundamentals/
│   │   └── README.md
│   ├── 02 - REST APIs/
│   │   └── README.md
│   ├── 03 - Axum/
│   │   └── README.md
│   ├── 04 - Actix Web/
│   │   └── README.md
│   ├── 05 - Routing/
│   │   └── README.md
│   ├── 06 - Middleware/
│   │   └── README.md
│   ├── 07 - Serialization (Serde)/
│   │   └── README.md
│   ├── 08 - Database integration/
│   │   ├── 01 - PostgreSQL/
│   │   │   └── README.md
│   │   ├── 02 - SQLx/
│   │   │   └── README.md
│   │   ├── 03 - Diesel ORM/
│   │   │   └── README.md
│   │   └── README.md
│   ├── 09 - Authentication/
│   │   └── README.md
│   ├── 10 - Authorization/
│   │   └── README.md
│   ├── 11 - WebSockets/
│   │   └── README.md
│   ├── 12 - gRPC/
│   │   └── README.md
│   ├── 13 - Caching/
│   │   └── README.md
│   ├── 14 - Background jobs/
│   │   └── README.md
│   └── README.md
├── 21 - Software Architecture/
│   ├── 01 - SOLID principles/
│   │   └── README.md
│   ├── 02 - Design patterns/
│   │   └── README.md
│   ├── 03 - Dependency injection/
│   │   └── README.md
│   ├── 04 - Layered architecture/
│   │   └── README.md
│   ├── 05 - Clean architecture/
│   │   └── README.md
│   ├── 06 - Hexagonal architecture/
│   │   └── README.md
│   ├── 07 - Domain-driven design/
│   │   └── README.md
│   ├── 08 - Modular monolith/
│   │   └── README.md
│   ├── 09 - Microservices/
│   │   └── README.md
│   ├── 10 - Event-driven architecture/
│   │   └── README.md
│   └── README.md
├── 22 - Performance Engineering/
│   ├── 01 - Memory optimization/
│   │   └── README.md
│   ├── 02 - CPU optimization/
│   │   └── README.md
│   ├── 03 - Allocation optimization/
│   │   └── README.md
│   ├── 04 - Cache locality/
│   │   └── README.md
│   ├── 05 - Profiling/
│   │   └── README.md
│   ├── 06 - Flame graphs/
│   │   └── README.md
│   ├── 07 - Benchmark analysis/
│   │   └── README.md
│   ├── 08 - SIMD/
│   │   └── README.md
│   ├── 09 - Release optimization/
│   │   └── README.md
│   ├── 10 - Binary size optimization/
│   │   └── README.md
│   └── README.md
├── 23 - Security Engineering/
│   ├── 01 - Memory safety guarantees/
│   │   └── README.md
│   ├── 02 - Input validation/
│   │   └── README.md
│   ├── 03 - Secure coding practices/
│   │   └── README.md
│   ├── 04 - Cryptography fundamentals/
│   │   └── README.md
│   ├── 05 - Secrets management/
│   │   └── README.md
│   ├── 06 - Dependency vulnerabilities/
│   │   └── README.md
│   ├── 07 - Supply chain security/
│   │   └── README.md
│   ├── 08 - Unsafe code auditing/
│   │   └── README.md
│   └── README.md
├── 24 - Production and DevOps/
│   ├── 01 - Environment configuration/
│   │   └── README.md
│   ├── 02 - Docker/
│   │   └── README.md
│   ├── 03 - CI and CD/
│   │   └── README.md
│   ├── 04 - GitHub Actions/
│   │   └── README.md
│   ├── 05 - Cloud deployment/
│   │   └── README.md
│   ├── 06 - Kubernetes/
│   │   └── README.md
│   ├── 07 - Observability/
│   │   └── README.md
│   ├── 08 - Health checks/
│   │   └── README.md
│   ├── 09 - Graceful shutdown/
│   │   └── README.md
│   ├── 10 - Production monitoring/
│   │   └── README.md
│   └── README.md
├── 25 - Specialization Paths/
│   ├── 01 - Backend engineering/
│   │   └── README.md
│   ├── 02 - Systems programming/
│   │   └── README.md
│   ├── 03 - Network programming/
│   │   └── README.md
│   ├── 04 - Embedded systems/
│   │   └── README.md
│   ├── 05 - Operating systems/
│   │   └── README.md
│   ├── 06 - CLI applications/
│   │   └── README.md
│   ├── 07 - Desktop applications (Tauri)/
│   │   └── README.md
│   ├── 08 - WebAssembly (WASM)/
│   │   └── README.md
│   ├── 09 - Blockchain/
│   │   └── README.md
│   ├── 10 - FinTech infrastructure/
│   │   └── README.md
│   ├── 11 - Game development/
│   │   └── README.md
│   ├── 12 - Cybersecurity/
│   │   └── README.md
│   ├── 13 - AI and ML Infrastructure/
│   │   └── README.md
│   └── README.md
├── 26 - Practical Projects/
│   ├── 01 - Beginner/
│   │   ├── 01 - Calculator/
│   │   │   ├── src/
│   │   │   │   └── main.rs
│   │   │   ├── .gitignore
│   │   │   ├── Cargo.lock
│   │   │   ├── Cargo.toml
│   │   │   └── README.md
│   │   ├── 02 - Guessing game/
│   │   │   └── README.md
│   │   ├── 03 - Temperature converter/
│   │   │   ├── src/
│   │   │   │   └── main.rs
│   │   │   ├── .gitignore
│   │   │   ├── Cargo.lock
│   │   │   ├── Cargo.toml
│   │   │   └── README.md
│   │   ├── 04 - CLI task manager/
│   │   │   └── README.md
│   │   └── README.md
│   ├── 02 - Intermediate/
│   │   ├── 01 - File organizer/
│   │   │   └── README.md
│   │   ├── 02 - Expense tracker/
│   │   │   └── README.md
│   │   ├── 03 - Multithreaded file processor/
│   │   │   └── README.md
│   │   ├── 04 - REST API with database/
│   │   │   └── README.md
│   │   └── README.md
│   ├── 03 - Advanced/
│   │   ├── 01 - Concurrent web server/
│   │   │   └── README.md
│   │   ├── 02 - Real-time chat application/
│   │   │   └── README.md
│   │   ├── 03 - Tauri desktop application/
│   │   │   └── README.md
│   │   ├── 04 - Distributed task queue/
│   │   │   └── README.md
│   │   └── README.md
│   ├── 04 - Professional/
│   │   ├── 01 - Production-grade backend/
│   │   │   └── README.md
│   │   ├── 02 - Payment processing system/
│   │   │   └── README.md
│   │   ├── 03 - High-performance caching server/
│   │   │   └── README.md
│   │   ├── 04 - Distributed microservices system/
│   │   │   └── README.md
│   │   ├── 05 - Open-source Rust contribution/
│   │   │   └── README.md
│   │   └── README.md
│   └── README.md
├── assets/
│   ├── rust-logo-white-outline.svg
│   └── rust-logo.svg
└── README.md
```

</details>

## How to use this diary

1. Follow the numbered sections in order.
2. Read each topic's `README.md` for explanations and examples.
3. Run the included Cargo projects or try the examples in a scratch Cargo
   project. Lessons identify when an example belongs in a library or needs
   additional dependencies.
4. Use `cargo check`, `cargo run`, `cargo fmt`, and `cargo clippy` while learning.

## Logo attribution

The Rust logo is © Rust Foundation and is provided by the [Rust Artwork repository](https://github.com/rust-lang/rust-artwork) under the Creative Commons Attribution license. It is used here to refer to the Rust programming language. Rust and Cargo are trademarks of the Rust Foundation. See the [Rust Logo Policy and Media Guide](https://rust-lang.org/policies/media-guide).
