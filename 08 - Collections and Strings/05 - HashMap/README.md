# HashMap

HashMap associates keys with values using hashing. Lookup order is unspecified. The entry API combines lookup and insertion for operations like counting.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    use std::collections::HashMap;
    let mut counts = HashMap::new();
    for word in ["oak", "elm", "oak"] {
        *counts.entry(word).or_insert(0) += 1;
    }
    assert_eq!(counts.get("oak"), Some(&2));
}
```

## 2. Common mistake

Never write logic that depends on a HashMap's iteration order.

## Practice

Count each word in a sentence, then sort the key-value pairs for predictable output.

## References

- [Official Rust documentation · collections](https://doc.rust-lang.org/std/collections/)

---

[← Previous: String vs &str](<../04 - String vs Str/README.md>) | [Back to Collections and Strings](<../README.md>) | [Next: HashSet →](<../06 - HashSet/README.md>)
