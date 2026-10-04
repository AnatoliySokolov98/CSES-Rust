# Two Knights — review

| Category | Score | Why |
|---|---|---|
| Code quality | 5/5 | Correct closed form, `i64` safe (k ≤ 10^4 → k^4 ≈ 10^16). |
| Readability | 3/5 | A one-line magic formula with no hint of where `(i - 1) * (i - 2) * 4` comes from. |
| Performance | 5/5 | O(1) per k. |

## Issues
- **The formula carries the whole solution and explains none of it.** In a month, or for a reviewer, `(i*i)*(i*i-1)/2 - (i-1)*(i-2)*4` is unverifiable without re-deriving it. Splitting it into named parts (`all_pairs`, `attacking_pairs`) plus one comment on the 2×3 block argument makes it checkable at a glance.

## Changes in improved.rs
- Named `squares`, `all_pairs` and `attacking_pairs`, with a comment explaining the attacking-pair count.
- Loop variable is `k`, since it's a board size, not an index.
