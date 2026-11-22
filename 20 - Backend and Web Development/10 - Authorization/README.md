# Authorization

Authorization decides whether an authenticated or anonymous principal may perform a particular operation. Apply the decision close to the resource boundary and deny when permission cannot be established.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn can_read(owner: u64, principal: u64) -> bool {
    owner == principal
}
assert!(can_read(7, 7));
assert!(!can_read(7, 9));
```

## 2. Common mistake

Authorizing a collection endpoint does not authorize access to every resource identifier.

## Practice

Test access with a resource owner, another user, and an explicitly privileged role.

## References

- [docs.rs · axum](https://docs.rs/axum/latest/axum/)

---

[← Previous: Authentication](<../09 - Authentication/README.md>) | [Back to Backend and Web Development](<../README.md>) | [Next: WebSockets →](<../11 - WebSockets/README.md>)
