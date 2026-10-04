const MOD: u64 = 1_000_000_007;

fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, usize);
    let mut res: u64 = 1;
    for _ in 0..n {
        res = res * 2 % MOD;
    }
    wln!(out, "{res}");
}
