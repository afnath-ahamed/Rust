# Destructuring

Destructuring pulls components out of tuples, structs, and enum variants. Wildcards skip values and .. can omit remaining fields. Matching through a reference can borrow the components.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point { x: 2, y: 8 };
    let Point { x, .. } = point;
    assert_eq!(x, 2);
    assert_eq!(point.y, 8);
}
```

## 2. Common mistake

Binding an owned non-Copy field by value can partially move the original struct.

## Practice

Destructure a borrowed struct containing a String and keep using the complete struct.

## References

- [Official Rust documentation · ch19 00 patterns](https://doc.rust-lang.org/book/ch19-00-patterns.html)

---

[← Previous: Let else](<../04 - Let else/README.md>) | [Back to Pattern Matching](<../README.md>) | [Next: Match guards →](<../06 - Match guards/README.md>)
