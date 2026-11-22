# Graphs

Graphs represent vertices connected by edges. An adjacency list scales with vertices and stored edges. Breadth-first search finds shortest paths in edge count for unweighted graphs.

## Contents

1. [Rust example](#1-rust-example)
2. [Common mistake](#2-common-mistake)

## 1. Rust example

```rust
fn main() {
    use std::collections::VecDeque;
    let edges = [vec![1, 2], vec![3], vec![3], vec![]];
    let mut queue = VecDeque::from([0]);
    let mut distance = [None; 4];
    distance[0] = Some(0);
    while let Some(node) = queue.pop_front() {
        for &next in &edges[node] {
            if distance[next].is_none() {
                distance[next] = distance[node].map(|d| d + 1);
                queue.push_back(next);
            }
        }
    }
    assert_eq!(distance[3], Some(2));
}
```

## 2. Common mistake

Mark on enqueue to avoid processing the same vertex repeatedly in cyclic graphs.

## Practice

Store predecessors and reconstruct a shortest path or report that the destination is unreachable.

## References

- [Official Rust documentation · collections](https://doc.rust-lang.org/std/collections/)

---

[← Previous: Trees](<../07 - Trees/README.md>) | [Back to Data Structures and Algorithms](<../README.md>) | [Next: Recursion →](<../09 - Recursion/README.md>)
