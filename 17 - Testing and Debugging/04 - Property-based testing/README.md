# Property-based testing

Property-based testing generates many inputs and looks for a counterexample to an invariant. A good property compares behavior to an independent rule and can be tested on randomly generated inputs.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
cargo add --dev proptest
```

```rust no_run
proptest::proptest! {
    #[test]
    fn reverse_twice_preserves_input(mut v in proptest::collection::vec(proptest::prelude::any::<u8>(), 0..100)) {
        let original = v.clone();
        v.reverse();
        v.reverse();
        proptest::prop_assert_eq!(v, original);
    }
}
```

## 2. Common mistake

If the property merely reimplements the tested algorithm, both may share the same bug.

## Practice

Specify a sort property that checks both order and preservation of elements.

## References

- [proptest-rs.github.io · proptest](https://proptest-rs.github.io/proptest/)
- [Official Rust documentation · ch11 00 testing](https://doc.rust-lang.org/book/ch11-00-testing.html)

---

[← Previous: Documentation testing](<../03 - Documentation testing/README.md>) | [Back to Testing and Debugging](<../README.md>) | [Next: Mocking →](<../05 - Mocking/README.md>)
