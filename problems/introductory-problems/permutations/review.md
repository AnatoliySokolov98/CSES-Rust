# Permutations — review

| Category | Score | Why |
|---|---|---|
| Code quality | 3/5 | Output format is wrong-looking, and passes only because the CSES checker compares tokens. |
| Readability | 4/5 | Evens-then-odds is clear, but the special case for `n == 1` suggests the construction doesn't handle it, when it does. |
| Performance | 5/5 | O(n). |

## Issues
- **Stray spaces and one number per line.** `wln!(out, " {i}")` and `"  {i}"` print a leading space (two for the odds), each on its own line. The problem asks for one line separated by spaces. CSES tolerates it, but a stricter checker, or a problem with an exact-format output, wouldn't. These look like debug leftovers.
- **Unneeded special case.** With `n == 1`, evens is empty and odds is `[1]`, so the general construction already prints `1`. The only real exceptions are 2 and 3.

## Changes in improved.rs
- Builds `evens.chain(odds)` and prints it with `w_vec!` on one line.
- The `NO SOLUTION` check is `n == 2 || n == 3`, with a comment on why the seam between halves is safe once n ≥ 4.
