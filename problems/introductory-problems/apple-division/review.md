# Apple Division — review

| Category | Score | Why |
|---|---|---|
| Code quality | 4/5 | Correct, but the loop bound quietly skips one mask and is only right because of a symmetry nobody wrote down. |
| Readability | 4/5 | Clear bitmask enumeration. |
| Performance | 5/5 | O(n · 2^n) ≈ 2 × 10^7 at n = 20; fine. |

## Issues
- **`0..(1 << n) - 1` skips the all-ones mask.** That's harmless, since all-ones gives the same difference as mask 0, but it reads like an off-by-one. If symmetry is the reason, use all of it: fix one apple's group and loop over `1 << (n - 1)` masks, which halves the work. Say why in a comment.
- **Two sums per mask when one suffices.** `second` is always `total - first`. Summing once up front makes each mask's work simpler, and the difference becomes `|total - 2·first|`.

## Changes in improved.rs
- Precomputes `total`, enumerates `1 << (n - 1)` masks, and computes one group sum per mask with an iterator. Comments explain the halving and the `total - 2·group` step.

## Better algorithm → optimal.rs
**Recursive include/exclude with a running sum**, O(2^n) instead of O(n · 2^n). Each mask in the bitmask loop re-sums up to n apples; carrying the sum down the recursion makes every subset cost O(1). It's also the template for meet-in-the-middle when n grows to 40.
