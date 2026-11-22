# Modular monolith

A modular monolith keeps one deployable application with deliberate internal boundaries. Rust modules or workspace packages can enforce narrow interfaces while allowing simple in-process calls.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
workspace
├── catalog/
├── orders/        # depends on CatalogRead interface
└── app/           # assembles both modules and owns process startup
```

## 2. Common mistake

Shared deployment is compatible with strict ownership of module data and behavior.

## Practice

Split one package into two collaborators and document their dependency direction.

## Related lesson

[Cargo workspaces](<../../10 - Project Organization/13 - Cargo workspaces/README.md>)

## References

- [Official Rust documentation · ch07 00 managing growing projects with packages crates and modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

[← Previous: Domain-driven design](<../07 - Domain-driven design/README.md>) | [Back to Software Architecture](<../README.md>) | [Next: Microservices →](<../09 - Microservices/README.md>)
