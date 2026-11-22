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
│   ├── 01 - Overview/
│   ├── 02 - Environment Setup/
│   └── 03 - First Rust Program/
├── 02 - Programming Fundamentals/
│   ├── 01 - Variables/
│   ├── 02 - Mutability/
│   ├── 03 - Constants/
│   ├── 04 - Shadowing/
│   ├── 05 - Data Types/
│   ├── 06 - Type Inference/
│   ├── 07 - Type Conversion/
│   ├── 08 - Operators/
│   ├── 09 - Functions/
│   ├── 10 - Expressions and Statements/
│   ├── 11 - Comments/
│   └── 12 - Control Flow/
├── 03 - Memory Management Fundamentals/
│   ├── 01 - Stack memory/
│   ├── 02 - Heap memory/
│   ├── 03 - Memory allocation/
│   ├── 04 - Memory deallocation/
│   ├── 05 - Pointers/
│   ├── 06 - Garbage collection/
│   ├── 07 - Dangling pointers/
│   ├── 08 - Memory leaks/
│   └── 09 - Memory safety/
├── 04 - Ownership System/
│   ├── 01 - Ownership rules/
│   ├── 02 - Variable scope/
│   ├── 03 - Move semantics/
│   ├── 04 - Copy semantics/
│   ├── 05 - Clone/
│   ├── 06 - Ownership with functions/
│   ├── 07 - Return values and ownership/
│   └── 08 - Drop/
├── 05 - Borrowing and References/
│   ├── 01 - References/
│   ├── 02 - Immutable borrowing/
│   ├── 03 - Mutable borrowing/
│   ├── 04 - Borrowing rules/
│   ├── 05 - Borrow checker/
│   ├── 06 - Dangling references/
│   ├── 07 - Dereferencing/
│   └── 08 - Slices/
├── 06 - Structs and Enums/
│   ├── 01 - Structs/
│   ├── 02 - Tuple structs/
│   ├── 03 - Unit-like structs/
│   ├── 04 - Methods/
│   ├── 05 - Associated functions/
│   ├── 06 - Implementation blocks (impl)/
│   ├── 07 - Enums/
│   ├── 08 - Option/
│   └── 09 - Result/
├── 07 - Pattern Matching/
│   ├── 01 - Match expressions/
│   ├── 02 - If let/
│   ├── 03 - While let/
│   ├── 04 - Let else/
│   ├── 05 - Destructuring/
│   ├── 06 - Match guards/
│   └── 07 - Exhaustive matching/
├── 08 - Collections and Strings/
│   ├── 01 - Arrays/
│   ├── 02 - Tuples/
│   ├── 03 - Vectors/
│   ├── 04 - String vs Str/
│   ├── 05 - HashMap/
│   ├── 06 - HashSet/
│   ├── 07 - BTreeMap/
│   ├── 08 - VecDeque/
│   └── 09 - UTF-8 encoding/
├── 09 - Error Handling/
│   ├── 01 - Recoverable errors/
│   ├── 02 - Unrecoverable errors/
│   ├── 03 - Panic/
│   ├── 04 - Result/
│   ├── 05 - Option/
│   ├── 06 - Unwrap and Expect/
│   ├── 07 - Error propagation/
│   ├── 08 - Custom errors/
│   ├── 09 - Thiserror/
│   └── 10 - Anyhow/
├── 10 - Project Organization/
│   ├── 01 - Packages/
│   ├── 02 - Crates/
│   ├── 03 - Modules/
│   ├── 04 - Module tree/
│   ├── 05 - Public and private visibility/
│   ├── 06 - Use keyword/
│   ├── 07 - Library crates/
│   ├── 08 - Binary crates/
│   ├── 09 - Cargo.toml/
│   ├── 10 - Cargo.lock/
│   ├── 11 - Dependencies/
│   ├── 12 - Features/
│   └── 13 - Cargo workspaces/
├── 11 - Generics, Traits and Lifetimes/
│   ├── 01 - Generic functions/
│   ├── 02 - Generic structs/
│   ├── 03 - Generic enums/
│   ├── 04 - Traits/
│   ├── 05 - Trait implementation/
│   ├── 06 - Trait bounds/
│   ├── 07 - Default implementations/
│   ├── 08 - Associated types/
│   ├── 09 - Lifetime annotations/
│   ├── 10 - Lifetime elision/
│   ├── 11 - Static lifetime/
│   ├── 12 - Trait objects/
│   └── 13 - Static vs Dynamic dispatch/
├── 12 - Functional Programming/
│   ├── 01 - Closures/
│   ├── 02 - Fn, FnMut and FnOnce/
│   ├── 03 - Iterators/
│   ├── 04 - Iterator adapters/
│   ├── 05 - Map/
│   ├── 06 - Filter/
│   ├── 07 - Fold/
│   ├── 08 - Collect/
│   ├── 09 - Lazy evaluation/
│   └── 10 - Zero-cost abstractions/
├── 13 - Smart Pointers/
│   ├── 01 - Box/
│   ├── 02 - Rc/
│   ├── 03 - Arc/
│   ├── 04 - RefCell/
│   ├── 05 - Cell/
│   ├── 06 - Weak/
│   ├── 07 - Cow/
│   ├── 08 - Deref trait/
│   ├── 09 - Drop trait/
│   ├── 10 - Interior mutability/
│   └── 11 - Reference cycles/
├── 14 - Concurrency and Parallelism/
│   ├── 01 - Processes vs Threads/
│   ├── 02 - Multithreading/
│   ├── 03 - Thread spawning/
│   ├── 04 - Move closures/
│   ├── 05 - Message passing/
│   ├── 06 - Channels/
│   ├── 07 - Shared state/
│   ├── 08 - Mutex/
│   ├── 09 - RwLock/
│   ├── 10 - Send and Sync traits/
│   ├── 11 - Atomics/
│   ├── 12 - Race conditions/
│   ├── 13 - Deadlocks/
│   └── 14 - Rayon/
├── 15 - Asynchronous Programming/
│   ├── 01 - Synchronous vs Asynchronous/
│   ├── 02 - Blocking vs Non-blocking/
│   ├── 03 - Async and Await/
│   ├── 04 - Futures/
│   ├── 05 - Executors/
│   ├── 06 - Async runtimes/
│   ├── 07 - Tokio/
│   ├── 08 - Async tasks/
│   ├── 09 - Async channels/
│   ├── 10 - Streams/
│   ├── 11 - Cancellation/
│   ├── 12 - Pin and Unpin/
│   └── 13 - Async error handling/
├── 16 - Advanced Rust/
│   ├── 01 - Unsafe Rust/
│   ├── 02 - Raw pointers/
│   ├── 03 - Unsafe functions/
│   ├── 04 - Advanced traits/
│   ├── 05 - Associated constants/
│   ├── 06 - Const generics/
│   ├── 07 - Type aliases/
│   ├── 08 - Type-state pattern/
│   ├── 09 - Higher-ranked trait bounds/
│   ├── 10 - Generic associated types/
│   ├── 11 - Declarative macros/
│   ├── 12 - Procedural macros/
│   ├── 13 - Foreign Function Interface (FFI)/
│   └── 14 - no_std Rust/
├── 17 - Testing and Debugging/
│   ├── 01 - Unit testing/
│   ├── 02 - Integration testing/
│   ├── 03 - Documentation testing/
│   ├── 04 - Property-based testing/
│   ├── 05 - Mocking/
│   ├── 06 - Fuzz testing/
│   ├── 07 - Benchmarking/
│   ├── 08 - Debugging/
│   ├── 09 - Logging/
│   ├── 10 - Tracing/
│   └── 11 - Code coverage/
├── 18 - Professional Rust Tooling/
│   ├── 01 - Cargo commands/
│   ├── 02 - Rustfmt/
│   ├── 03 - Clippy/
│   ├── 04 - Rustdoc/
│   ├── 05 - Cargo Audit/
│   ├── 06 - Cargo Deny/
│   ├── 07 - Miri/
│   ├── 08 - Compiler diagnostics/
│   ├── 09 - Build profiles/
│   ├── 10 - Cross-compilation/
│   └── 11 - Publishing crates/
├── 19 - Data Structures and Algorithms/
│   ├── 01 - Time and space complexity/
│   ├── 02 - Searching algorithms/
│   ├── 03 - Sorting algorithms/
│   ├── 04 - Linked lists/
│   ├── 05 - Stacks/
│   ├── 06 - Queues/
│   ├── 07 - Trees/
│   ├── 08 - Graphs/
│   ├── 09 - Recursion/
│   └── 10 - Dynamic programming/
├── 20 - Backend and Web Development/
│   ├── 01 - HTTP fundamentals/
│   ├── 02 - REST APIs/
│   ├── 03 - Axum/
│   ├── 04 - Actix Web/
│   ├── 05 - Routing/
│   ├── 06 - Middleware/
│   ├── 07 - Serialization (Serde)/
│   ├── 08 - Database integration/
│   ├── 09 - Authentication/
│   ├── 10 - Authorization/
│   ├── 11 - WebSockets/
│   ├── 12 - gRPC/
│   ├── 13 - Caching/
│   └── 14 - Background jobs/
├── 21 - Software Architecture/
│   ├── 01 - SOLID principles/
│   ├── 02 - Design patterns/
│   ├── 03 - Dependency injection/
│   ├── 04 - Layered architecture/
│   ├── 05 - Clean architecture/
│   ├── 06 - Hexagonal architecture/
│   ├── 07 - Domain-driven design/
│   ├── 08 - Modular monolith/
│   ├── 09 - Microservices/
│   └── 10 - Event-driven architecture/
├── 22 - Performance Engineering/
│   ├── 01 - Memory optimization/
│   ├── 02 - CPU optimization/
│   ├── 03 - Allocation optimization/
│   ├── 04 - Cache locality/
│   ├── 05 - Profiling/
│   ├── 06 - Flame graphs/
│   ├── 07 - Benchmark analysis/
│   ├── 08 - SIMD/
│   ├── 09 - Release optimization/
│   └── 10 - Binary size optimization/
├── 23 - Security Engineering/
│   ├── 01 - Memory safety guarantees/
│   ├── 02 - Input validation/
│   ├── 03 - Secure coding practices/
│   ├── 04 - Cryptography fundamentals/
│   ├── 05 - Secrets management/
│   ├── 06 - Dependency vulnerabilities/
│   ├── 07 - Supply chain security/
│   └── 08 - Unsafe code auditing/
├── 24 - Production and DevOps/
│   ├── 01 - Environment configuration/
│   ├── 02 - Docker/
│   ├── 03 - CI and CD/
│   ├── 04 - GitHub Actions/
│   ├── 05 - Cloud deployment/
│   ├── 06 - Kubernetes/
│   ├── 07 - Observability/
│   ├── 08 - Health checks/
│   ├── 09 - Graceful shutdown/
│   └── 10 - Production monitoring/
├── 25 - Specialization Paths/
│   ├── 01 - Backend engineering/
│   ├── 02 - Systems programming/
│   ├── 03 - Network programming/
│   ├── 04 - Embedded systems/
│   ├── 05 - Operating systems/
│   ├── 06 - CLI applications/
│   ├── 07 - Desktop applications (Tauri)/
│   ├── 08 - WebAssembly (WASM)/
│   ├── 09 - Blockchain/
│   ├── 10 - FinTech infrastructure/
│   ├── 11 - Game development/
│   ├── 12 - Cybersecurity/
│   └── 13 - AI and ML Infrastructure/
└── 26 - Practical Projects/
    ├── 01 - Beginner/
    ├── 02 - Intermediate/
    ├── 03 - Advanced/
    └── 04 - Professional/
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
