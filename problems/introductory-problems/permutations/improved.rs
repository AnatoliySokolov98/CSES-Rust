fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, usize);
    if n == 2 || n == 3 {
        wln!(out, "NO SOLUTION");
        return;
    }
    // All evens, then all odds: neighbors inside each half differ by 2, and the
    // seam (largest even, then 1) differs by at least 3 once n >= 4.
    let permutation: Vec<usize> = (2..=n).step_by(2).chain((1..=n).step_by(2)).collect();
    w_vec!(out, permutation);
}
