# Module tree

A module tree begins at the crate root. Child declarations establish nesting, and file layout follows that nesting when modules are out of line.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
src/lib.rs          declares mod catalog;
src/catalog.rs      declares mod product;
src/catalog/product.rs defines the product module
```

## 2. Common mistake

A physical project hierarchy may contain assets and tests that are outside the module tree.

## Practice

Draw the fully qualified path to one public function housed in product.

## References

- [Official Rust documentation · ch07 00 managing growing projects with packages crates and modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

[← Previous: Modules](<../03 - Modules/README.md>) | [Back to Project Organization](<../README.md>) | [Next: Public and private visibility →](<../05 - Public and private visibility/README.md>)
