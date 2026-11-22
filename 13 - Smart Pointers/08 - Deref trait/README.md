# Deref trait

Deref lets a pointer-like type expose a target reference and participate in deref coercion. For example, &String can be passed where &str is expected.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    fn first(text: &str) -> Option<char> {
        text.chars().next()
    }
    let owned = String::from("rust");
    assert_eq!(first(&owned), Some('r'));
}
```

## 2. Common mistake

Avoid implementing Deref just to imitate inheritance or hide expensive work.

## Practice

Identify each deref coercion when passing &Box<String> to first.

## References

- [Official Rust documentation · ch15 00 smart pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)

---

[← Previous: `Cow<T>`](<../07 - Cow/README.md>) | [Back to Smart Pointers](<../README.md>) | [Next: Drop trait →](<../09 - Drop trait/README.md>)
