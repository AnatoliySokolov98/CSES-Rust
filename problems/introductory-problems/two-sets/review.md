# Two Sets — review

| Category | Score | Why |
|---|---|---|
| Code quality | 5/5 | Greedy from the top is correct, and the sums fit easily (≈ 5 × 10^11). |
| Readability | 5/5 | `if i <= half` makes the greedy obvious. |
| Performance | 5/5 | O(n). |

## Issues
None. The greedy (take `i` while it fits in the remaining half) is the neatest approach to this problem.

## Changes in improved.rs
None; it's a copy of the original.
