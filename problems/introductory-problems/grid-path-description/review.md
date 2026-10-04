# Grid Path Description — review

| Category | Score | Why |
|---|---|---|
| Code quality | 2/5 | `solution.rs` doesn't compile as committed, and the four move cases are copy-pasted. |
| Readability | 3/5 | The prune is right, but it's spread over counters and magic numbers (6, 7, 48). |
| Performance | 3/5 | Passed, but ~0.20s locally on the all-`?` input, which is close to the limit on a slower judge. |

## Issues
- **The committed file doesn't compile.** `solve` uses `N`, but `const N` isn't defined, so `./cses test` fails at build time. The version submitted to CSES must have been different. Keep the repo in sync with what passed.
- **Four copy-pasted move blocks.** Each direction repeats the same bounds check and recursive call with different offsets. A table of `(drow, dcol, letter)` and one loop removes about 30 lines and makes adding a check (like a new prune) a one-place change.
- **Bounds checks everywhere.** `nr >= 0 && nr < 7 && nc >= 0 && nc < 7`, the `col == 0 ||` checks in the prune, and the `as usize` casts all exist because the grid has edges. A **padded grid**, 9×9 with the border pre-marked as visited, makes the edges look like visited cells. All those checks disappear, and the result is also faster.
- **Types:** `&Vec<char>` should be `&[u8]`. `&mut [[bool; 7]]` drops the row count from the type. Returning `i32` for a count works (max 88,418) but `u64` is the safe default for counts.
- **Magic numbers:** `6`, `7` and `48` should come from `N`: `N - 1` and `N * N - 1`.

## Changes in improved.rs
Same algorithm and same prune:
- 9×9 padded `visited` grid; moves via `wrapping_add` on a `MOVES` table.
- The split prune is written as `left == right && up == down && left != up` ("one axis fully blocked, the other fully open"), with a comment.
- Runs in **0.15s** vs **0.20s** for the original on the all-`?` input: cleaner and faster.

## Better algorithm → optimal.rs
**Adds the dead-end (forced move) prune.** Once the current cell is marked visited, a free neighbor with at most one other free neighbor has to be entered next, or it becomes a cell you can enter but never leave. Two such neighbors means the path is dead. This cuts the search tree by more than 10×: the all-`?` input runs in **0.02s**.
