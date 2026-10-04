# MEX Grid Construction — review

| Category | Score | Why |
|---|---|---|
| Code quality | 4/5 | Correct greedy, but uses `HashSet`s for a small, dense range of integers. |
| Readability | 4/5 | Clear direct simulation of the definition. |
| Performance | 5/5 | ~2 × 10^6 hash lookups at n = 100; fine. |

## Issues
- **`HashSet` where a boolean table fits.** Each value is the mex of at most `2(n - 1)` earlier values, so it's below `2n`. When keys are small integers with a known bound, `Vec<bool>` (or `[bool; K]`) is simpler and an order of magnitude faster than hashing. That's worth reaching for by default.

## Changes in improved.rs
- `in_row` / `in_col` are `n × 2n` boolean tables, and the mex is found with `(0..).find(...)`.

## Better algorithm → optimal.rs
**Answer is `row ^ col`.** The value at (r, c), defined as the mex of everything to the left and above, is the Nim sum of r and c (the same reason the mex of a Nim position equals the XOR of the pile sizes). That's O(1) per cell with no state. It's worth knowing because "mex of row and column" grids, and Sprague–Grundy questions generally, keep turning into XOR.
