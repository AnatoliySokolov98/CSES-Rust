# Palindrome Reorder — review

| Category | Score | Why |
|---|---|---|
| Code quality | 3/5 | Correct, but uses a magic sentinel and two-pointer bookkeeping that the idea doesn't need. |
| Readability | 3/5 | The odd letter is found twice (counted, then located again), and the fill loop hides the "half + middle + mirror" shape. |
| Performance | 5/5 | O(n). |

## Issues
- **Sentinel value `26` for "no odd letter".** `odd_index = 26` followed by `if odd_index != 26` is a hand-rolled `Option`. If you just collect the odd letters into a small list, that list's length gives the `NO SOLUTION` test, and its contents give the middle letter.
- **Odd letters are handled in two separate places.** The first loop counts them; the fill loop rediscovers which one it was.
- **Two-pointer fill with `r -= 1`.** It works (`r` never underflows because the pairs run out first), but you have to reason about that to trust it. Building the left half, then pushing the middle, then the reversed half has no index arithmetic at all.
- **Minor:** `for char in s` names a variable `char`, which shadows the type name.

## Changes in improved.rs
- Counts letters, collects `odd` letters into a `Vec`, and returns `NO SOLUTION` if there's more than one.
- Builds `half` (each letter `count / 2` times, sorted), then `half + odd + reversed(half)`.
