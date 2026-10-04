# Gray Code — review

| Category | Score | Why |
|---|---|---|
| Code quality | 4/5 | Correct reflect-and-prefix construction. |
| Readability | 3/5 | Ten lines of manual bit-by-bit printing that a format specifier does in one. |
| Performance | 5/5 | 2^16 codes; fine. |

## Issues
- **Hand-written binary printing.** The inner loop masks each bit and writes `"0"` or `"1"` one call at a time. Rust's formatter does this directly: `{:0width$b}` prints a number in binary, zero-padded to `width`. Knowing the formatting mini-language (`{:b}`, `{:x}`, `{:>8}`, `{:.3}`) saves a lot of code in CP output.
- **Minor:** `vec![0, 1]` seeds the first bit by hand, so the loop starts at `1`. Starting from `vec![0]` and looping over every bit is more uniform.

## Changes in improved.rs
- Same reflection, seeded with `[0]` and looping over all `n` bits, with a comment describing the reflection.
- Prints each code with `{:0width$b}`.

## Better algorithm → optimal.rs
**Closed form `i ^ (i >> 1)`.** The i-th reflected Gray code can be computed directly, with no list and no reflection: one loop, O(1) memory. It's worth memorizing; it also shows up in "enumerate subsets changing one element at a time" tricks.
