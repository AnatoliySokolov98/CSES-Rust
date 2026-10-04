# Weird Algorithm — review

| Category | Score | Why |
|---|---|---|
| Code quality | 4/5 | Correct, but the type choice depends on the platform being 64-bit. |
| Readability | 5/5 | Reads exactly like the problem statement. |
| Performance | 5/5 | Simulation is the intended solution. |

## Issues
- **`usize` for values that pass 2^32.** Starting below 10^6, the sequence peaks around 5.7 × 10^10. `usize` is 64 bits on CSES, so this passes, but on a 32-bit target `n * 3 + 1` would overflow. When a value has a known range, pick the type from the range (`u64`), not from "it's an index-ish number" (`usize`).

## Changes in improved.rs
- `n` is read as `u64`, with a comment noting the peak value.
