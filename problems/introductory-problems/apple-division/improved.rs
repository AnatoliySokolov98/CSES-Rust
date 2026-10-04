fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, usize);
    let weights: Vec<i64> = (0..n).map(|_| read!(it, i64)).collect();
    let total: i64 = weights.iter().sum();
    let mut best = i64::MAX;
    // A split and its mirror image give the same difference, so keep the last apple
    // out of the first group: only half the masks need checking.
    for mask in 0..1usize << (n - 1) {
        let group: i64 = (0..n).filter(|&i| mask >> i & 1 == 1).map(|i| weights[i]).sum();
        // The other group weighs total - group.
        best = best.min((total - 2 * group).abs());
    }
    wln!(out, "{best}");
}
