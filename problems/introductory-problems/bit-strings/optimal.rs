const MOD: u64 = 1_000_000_007;

fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, u64);
    wln!(out, "{}", pow_mod(2, n));
}

// Binary exponentiation: square the base once per bit of exp, O(log exp).
fn pow_mod(mut base: u64, mut exp: u64) -> u64 {
    let mut res = 1;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            res = res * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    res
}
