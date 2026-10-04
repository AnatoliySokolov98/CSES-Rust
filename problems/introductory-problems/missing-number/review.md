# Missing Number — review

| Category | Score | Why |
|---|---|---|
| Code quality | 5/5 | No stored array, no overflow risk (`n(n+1)/2` ≤ 2 × 10^10 in 64 bits). |
| Readability | 5/5 | One idea, three lines. |
| Performance | 5/5 | O(n), O(1) memory. |

## Issues
None.

## Changes in improved.rs
None; it's a copy of the original. The sum formula is the standard approach. XOR of 1..n against the input is the alternative people mention, but it isn't better here.
