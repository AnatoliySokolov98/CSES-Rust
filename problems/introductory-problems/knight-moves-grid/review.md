# Knight Moves Grid — review

| Category | Score | Why |
|---|---|---|
| Code quality | 5/5 | Textbook BFS; `usize::MAX` as "unvisited" doubles as the distance table. |
| Readability | 5/5 | Short and conventional. |
| Performance | 5/5 | O(n²), which is the output size, so nothing can beat it. |

## Issues
None worth changing. The `i32` round-trip for neighbor coordinates is the usual cost of signed offsets in Rust. The `wrapping_add` + bounds-check trick (used in grid-path-description/improved.rs) is an alternative, but it isn't clearer here.

## Changes in improved.rs
None; it's a copy of the original.
