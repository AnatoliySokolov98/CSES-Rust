fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, usize);
    let modulo = 1_000_000_007;
    let mut res = 1;
    for _ in 0..n {
        res = (res * 2) % modulo;
    }
    wln!(out, "{res}");
}
