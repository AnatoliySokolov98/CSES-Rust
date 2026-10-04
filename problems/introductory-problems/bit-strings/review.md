# Bit Strings — review

| Category | Score | Why |
|---|---|---|
| Code quality | 4/5 | Correct, but `res` and `modulo` are inferred as `i32`, which only just fits. |
| Readability | 5/5 | Clear. |
| Performance | 5/5 | O(n) with n ≤ 10^6 is fine. |

## Issues
- **Inferred `i32` that's one step from overflowing.** With no type annotation, Rust makes `res` and `modulo` `i32`. `res * 2` peaks near 2.0 × 10^9, just under `i32::MAX` (2.147 × 10^9). Change the 2 to a 3, or use this as a template for multiplying two residues, and it overflows: a panic in debug, silent wraparound in release. Make modular arithmetic `u64` (or `i64`) by default, with the modulus as a `const`.

## Changes in improved.rs
- `const MOD: u64 = 1_000_000_007;` and `res: u64`.

## Better algorithm → optimal.rs
**Binary exponentiation**, O(log n) instead of O(n). It doesn't matter at n ≤ 10^6, but `pow_mod` is a staple: it's needed as soon as the exponent is 10^18, and for modular inverses (`pow_mod(a, MOD - 2)`). Worth having ready to type from memory.
