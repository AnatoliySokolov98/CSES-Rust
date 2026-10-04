fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, usize);
    // Every element has to be raised to the largest value seen so far.
    let mut max = 0;
    let mut moves: u64 = 0;
    for _ in 0..n {
        let x = read!(it, u64);
        max = max.max(x);
        moves += max - x;
    }
    wln!(out, "{moves}");
}
