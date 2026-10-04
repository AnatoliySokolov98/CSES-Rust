# Tower of Hanoi — review

| Category | Score | Why |
|---|---|---|
| Code quality | 4/5 | Correct recursion, but stores every move just to count them, when the count is known. |
| Readability | 4/5 | `hanoi(1, first, last, middle, res)` is a recursive call standing in for "record one move". |
| Performance | 5/5 | 2^16 moves; fine. |

## Issues
- **Collecting the moves to learn their count.** The minimum is always `2^n - 1`, so you can print the count first and write each move as it happens, with no `Vec`.
- **Recursing to record a single move.** The middle step calls `hanoi(1, ...)`, which hits the `n == 1` base case and pushes one pair. Writing the move directly makes the three steps of the algorithm (move n-1 away, move the largest disk, move n-1 back) visible. With a base case of `n == 0` (do nothing), the `n == 1` case is no longer special.
- **Minor:** `first` / `last` / `middle` describe positions, not roles. `from` / `to` / `via` read better in the recursive calls.

## Changes in improved.rs
- Prints `(1 << n) - 1` up front and writes each move straight to `out` from inside `hanoi`.
- Base case `n == 0`; the middle step is a direct `wln!`; parameters are `from`, `to`, `via`.
