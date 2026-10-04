# Number Spiral — review

| Category | Score | Why |
|---|---|---|
| Code quality | 4/5 | Correct and overflow-safe (values up to ~10^18 in 64 bits). |
| Readability | 3/5 | Four branches that are two mirrored pairs, a misleading name, and parity tested on 0-indexed coordinates. |
| Performance | 5/5 | O(1) per query. |

## Issues
- **Four branches for one formula.** On an odd layer `k`, the value is `corner + x - y` whether you're in the row or the column part; on an even layer it's `corner + y - x`. The `row < col` split and the inner parity checks rebuild that same rule four times.
- **`square` isn't a square.** It holds `k^2 - k + 1`, the value on the diagonal corner of layer `k`. A name like `corner` says what it is.
- **0-indexing obscures the parity.** After `row -= 1`, the check `col % 2 == 0` really means "the layer is odd". Keeping the 1-indexed coordinates the problem gives you makes the parity match the picture.
- **Minor:** the query count in `solve` is called `n`; `t` is the convention, and `n` reads like a grid size.

## Changes in improved.rs
- Single formula: `k = max(y, x)`, `corner = k² - k + 1`, then `corner ± (x - y)` depending on the parity of `k`. Uses `i64` so the signed difference is natural.
- Comments explain what `corner` is and which way each layer runs.
