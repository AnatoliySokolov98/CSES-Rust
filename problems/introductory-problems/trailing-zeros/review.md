# Trailing Zeros — review

| Category | Score | Why |
|---|---|---|
| Code quality | 5/5 | Correct counting of factors of 5. `curr *= 5` can't overflow a 64-bit `usize` for n ≤ 10^9. |
| Readability | 5/5 | Standard form of Legendre's formula. |
| Performance | 5/5 | O(log n). |

## Issues
None.

## Changes in improved.rs
None; it's a copy of the original.
