# Grid Coloring I — review

| Category | Score | Why |
|---|---|---|
| Code quality | 4/5 | Correct greedy, but allocates a `Vec` for every cell to hold at most 3 letters. |
| Readability | 4/5 | Clear, though it never says why `IMPOSSIBLE` can't happen, which is the key fact. |
| Performance | 5/5 | 250k small allocations at 500×500 is still fast. |

## Issues
- **A heap allocation per cell.** `skipped` is a fresh `Vec` for up to 3 bytes, 250,000 times. Comparing against `up`, `left` and `original` directly, with a placeholder for missing neighbors, needs no collection at all.
- **The proof is the solution, and it's missing.** The greedy works because at most 3 of the 4 letters are ever ruled out, so `IMPOSSIBLE` is never the answer. A one-line comment tells the reader why there's no `IMPOSSIBLE` branch.
- **Types:** `Vec<Vec<char>>` collected back into `String` per row; bytes and `from_utf8` are lighter.

## Changes in improved.rs
- Grid stored as bytes; each cell picks the first of `b"ABCD"` that differs from `original`, `up` and `left`, with a comment explaining why one always exists.
