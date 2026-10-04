# Raab Game I — review

| Category | Score | Why |
|---|---|---|
| Code quality | 3/5 | Correct, but `n` is mutated into a cursor and then reused with a different meaning. |
| Readability | 3/5 | The construction is hard to verify, and the `NO` condition has no explanation. |
| Performance | 5/5 | O(n) per test. |

## Issues
- **`n` changes meaning halfway through.** After the ties loop decrements it, `n` is no longer the number of rounds but "the largest card not yet used" (= a + b). The next two loops depend on that. To check them you have to track the mutation in your head. Naming the value (`m = a + b`) and leaving `n` alone makes each loop checkable on its own.
- **Three loops, each pushing to both vectors.** A cleaner construction: player 1 plays card `k` in round `k`, and player 2 plays a *cyclic shift* of the cards `1..=a+b`. Shifting up by `a` makes player 2 win the first `b` rounds and lose the next `a`. One formula then defines the whole answer.
- **The `NO` condition is unexplained.** `(a > 0 && b == 0) || (b > 0 && a == 0)` is `(a == 0) != (b == 0)`. The reason is the real insight: both players hold the same cards, so the sum of card differences is 0, and wins for one side have to be balanced by wins for the other.
- **Shadowing:** `solve` calls its test count `n`, and `solve_once` uses `n` for something else.

## Changes in improved.rs
- `t` for the test count.
- `NO` check written as `a + b > n || (a == 0) != (b == 0)`, with the sum argument in a comment.
- Player 1 plays `1..=n`; player 2 plays `k + a` for `k ≤ b`, `k - b` for `b < k ≤ a + b`, and `k` (a tie) above that.
