# Creating Strings — review

| Category | Score | Why |
|---|---|---|
| Code quality | 3/5 | Generates every ordering including duplicates, then removes them with a `HashSet` and fixes the order with a sort. |
| Readability | 4/5 | Standard backtracking, easy to follow. |
| Performance | 4/5 | Fine at n ≤ 8, but the work doesn't scale with the output: `"aaaaaaaa"` builds 40,320 strings to print one. |

## Issues
- **Duplicates are created and then removed.** The standard fix is to sort the letters first, then skip a letter if it equals the previous letter and that previous copy isn't in use. That makes equal letters get used in a fixed order, so each distinct string is generated exactly once.
- **The sort is a symptom.** Once the input is sorted and duplicates are skipped, backtracking already produces strings in lexicographic order. That removes both the `HashSet` and the final sort.
- **Signature style:** `&Vec<char>` / `&mut Vec<bool>` should be `&[u8]` / `&mut [bool]`. Slices accept more callers, and bytes avoid the 4-byte `char`.

## Changes in improved.rs
- Sorts the bytes, skips duplicate letters in the loop (with a comment explaining the rule), and collects results straight into a `Vec<String>` that's already in order.

## Better algorithm → optimal.rs
**`next_permutation`.** Starting from the sorted string, repeatedly step to the next larger arrangement until there isn't one. It handles duplicates naturally, uses no recursion and no `used` array, and is O(n) per output string. It's the same function as C++'s `std::next_permutation`; Rust's standard library doesn't have one, so it's worth knowing how to write.
