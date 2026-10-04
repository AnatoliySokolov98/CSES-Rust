fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, i64);
    for k in 1..=n {
        let squares = k * k;
        let all_pairs = squares * (squares - 1) / 2;
        // Two knights attack each other exactly when they sit in opposite corners of a
        // 2x3 or 3x2 block. There are 2(k-1)(k-2) such blocks, each with 2 attacking pairs.
        let attacking_pairs = 4 * (k - 1) * (k - 2);
        wln!(out, "{}", all_pairs - attacking_pairs);
    }
}
