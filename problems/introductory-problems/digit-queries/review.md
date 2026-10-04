# Digit Queries — review

| Category | Score | Why |
|---|---|---|
| Code quality | 5/5 | Correct block-skipping; `count * digits` stays within u64 for k ≤ 10^18. |
| Readability | 4/5 | The final digit extraction (count back from the right, divide in a loop) takes a moment to verify. |
| Performance | 5/5 | O(log k) per query. |

## Issues
- **Digit extraction from the right.** `loc = (digits - 1) - k % digits` followed by repeated division converts "index from the left" to "index from the right" and back. `number.to_string().as_bytes()[k % digits]` indexes from the left directly, at a cost that's irrelevant here (≤ 19 digits, 1000 queries).
- **Minor:** `nums` / `start` are clearer as `count` / `first`, and `k -= 1` deserves a comment saying the position becomes 0-indexed.

## Changes in improved.rs
- Renamed `nums` → `count` and `start` → `first`, and added comments for the 0-indexing and the block skipping.
- The digit is taken by indexing the decimal string.
