# Chessboard and Queens — review

| Category | Score | Why |
|---|---|---|
| Code quality | 3/5 | Correct, but threads three `&mut Vec<bool>` plus the board through a 5-parameter recursion with manual undo. |
| Readability | 4/5 | The structure is clear; magic numbers (8, 16, `8 + row - col`) need decoding. |
| Performance | 5/5 | Tiny search space. |

## Issues
- **Mutable shared state with manual undo.** Every placement sets three flags and has to clear exactly those three after the call. That's easy to get wrong when the state grows. Bitmasks passed **by value** remove the undo step entirely: the recursive call gets `cols | bit`, and the caller's masks are untouched.
- **Magic numbers.** `8` appears throughout, the diagonal arrays have length `16` (15 are used, index 0 never is), and `8 + row - col` has an offset you have to verify. One `const N` and the standard `row + col` / `row - col + N - 1` indexing fixes this.
- **Types:** `Vec<Vec<char>>` and `&mut Vec<bool>` should be `Vec<Vec<u8>>` and slices.

## Changes in improved.rs
- `const N: usize = 8`; the board is read as bytes.
- `count(board, row, cols, diags, antis)` takes `u32` masks by value; a comment explains which bit means what. No undo step.
