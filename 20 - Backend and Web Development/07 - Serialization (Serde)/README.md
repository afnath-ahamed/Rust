# Serialization (Serde)

Serde derives conversions between Rust types and data formats. Use distinct request and response types when accepted fields differ from stored or returned fields.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```shell
cargo add serde --features derive
cargo add serde_json
```

```rust no_run
fn main() {
    #[derive(serde::Deserialize, serde::Serialize)]
    struct Note {
        title: String,
    }
    let note: Note = serde_json::from_str(r#"{"title":"Rust"}"#).unwrap();
    assert_eq!(note.title, "Rust");
}
```

## 2. Common mistake

Successful deserialization does not enforce business invariants such as empty or overlong titles.

## Practice

Parse a request into a DTO, validate it, and return field-specific errors.

## References

- [serde.rs](https://serde.rs/)
- [docs.rs · axum](https://docs.rs/axum/latest/axum/)

---

[← Previous: Middleware](<../06 - Middleware/README.md>) | [Back to Backend and Web Development](<../README.md>) | [Next: Database integration →](<../08 - Database integration/README.md>)
