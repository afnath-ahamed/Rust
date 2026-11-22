# Documentation testing

Rustdoc compiles documentation examples as tests. Use them to demonstrate the public API with asserted behavior and the correct crate import. Use compile_fail for intentional rejection examples.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```text
/// Converts whole minutes to seconds.
///
/// ```
/// assert_eq!(time_utils::seconds(2), 120);
/// ```
```

```shell
cargo test --doc
```

## 2. Common mistake

A copied code block with a missing import can become broken documentation, so test examples with the consuming crate path.

## Practice

Add a doctest to a public function and run it separately.

## Related lesson

[Rustdoc](<../../18 - Professional Rust Tooling/04 - Rustdoc/README.md>)

## References

- [Official Rust documentation · ch11 00 testing](https://doc.rust-lang.org/book/ch11-00-testing.html)

---

[← Previous: Integration testing](<../02 - Integration testing/README.md>) | [Back to Testing and Debugging](<../README.md>) | [Next: Property-based testing →](<../04 - Property-based testing/README.md>)
