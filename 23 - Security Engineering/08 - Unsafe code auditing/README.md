# Unsafe code auditing

Review each unsafe operation against its documented invariants. Trace inputs through bounds, alignment, initialization, lifetime, aliasing, and ownership as applicable. Check both error and drop paths.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
Review an unsafe FFI wrapper: caller inputs → validation → foreign contract → result conversion → cleanup
Record which invariant is proven at each boundary, then test realistic valid and invalid inputs.
```

## 2. Common mistake

Tests and tools such as Miri provide evidence for covered behavior. They do not prove soundness over every possible safe call.

## Practice

Audit a small wrapper in your own code and identify every condition the safe interface must establish.

## Related lesson

[Unsafe Rust](<../../16 - Advanced Rust/01 - Unsafe Rust/README.md>)

## References

- [rustsec.org](https://rustsec.org/)

---

[← Previous: Supply chain security](<../07 - Supply chain security/README.md>) | [Back to Security Engineering](<../README.md>) | [Next: Production and DevOps →](<../../24 - Production and DevOps/README.md>)
