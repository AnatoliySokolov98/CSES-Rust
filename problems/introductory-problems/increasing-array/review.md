# Increasing Array — review

| Category | Score | Why |
|---|---|---|
| Code quality | 4/5 | Correct, `i64` sized for the answer, but stores and mutates the whole input to carry one number forward. |
| Readability | 4/5 | `nums[i] = nums[i - 1]` hides the real idea: everything rises to the running maximum. |
| Performance | 5/5 | O(n). |

## Issues
- **The array is only used as a carrier for the running max.** Writing `nums[i] = nums[i - 1]` back into the input is a roundabout way of tracking one variable. Once you see that, the solution streams: read a number, update the max, add the gap. That leaves no `Vec` and no mutated input, and the loop body states the whole idea.

## Changes in improved.rs
- Streams the input with a running `max` and adds `max - x` per element; a comment states the idea.
- `u64` for the total, since all values are positive.
