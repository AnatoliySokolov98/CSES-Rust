# String Reorder — review

| Category | Score | Why |
|---|---|---|
| Code quality | 3/5 | Correct greedy, but the control flow (a flag, two loops, `continue`) and the `prev` handling are fragile. |
| Readability | 2/5 | `rest` means two different things, and `prev.is_none() \|\| i as u8 != (prev.unwrap() - b'A')` packs a lot into one condition. |
| Performance | 5/5 | O(26n) at n = 10^6. |

## Issues
- **A flag to emulate "try this, else that".** `found_max` plus `continue` implements "pick the forced letter if there is one, otherwise the smallest letter that differs from the previous one". `find(...).or_else(|| find(...))` says exactly that in one expression.
- **`rest` is reused with a different meaning.** Outside the loop it's "letters other than the most common"; inside, a new `rest` is "remaining slots minus this letter". Shadowing a name with a related-but-different meaning invites bugs.
- **`prev` is stored as a byte but compared as an index.** Every comparison converts with `- b'A'`, after an `is_none()` / `unwrap()` pair. Keeping `prev: Option<usize>` (the letter index) lets you compare with `Some(i) != prev`.
- **The forcing rule isn't explained.** It's the heart of the solution: a letter that fills more than half of the remaining slots has to be placed now. It deserves a comment.

## Changes in improved.rs
- Feasibility check written as `2 * max > len + 1`, with a comment.
- One loop over the remaining-slot count. It picks the forced letter or the smallest valid one with `find().or_else()` and tracks `prev` as `Option<usize>`.
