# VecDeque

VecDeque is a growable ring buffer. It supports efficient insertion and removal at both ends, making it useful as a FIFO queue or a sliding window.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    use std::collections::VecDeque;
    let mut jobs = VecDeque::from([7, 8]);
    jobs.push_back(9);
    assert_eq!(jobs.pop_front(), Some(7));
}
```

## 2. Common mistake

The logical order can occupy two storage slices, so it is not always one contiguous slice.

## Practice

Keep only the most recent three readings using push_back and pop_front.

## References

- [Official Rust documentation · collections](https://doc.rust-lang.org/std/collections/)

---

[← Previous: BTreeMap](<../07 - BTreeMap/README.md>) | [Back to Collections and Strings](<../README.md>) | [Next: UTF-8 encoding →](<../09 - UTF-8 encoding/README.md>)
