fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, usize);
    // The greedy grid is exactly row ^ col: it's the Nim sum of two piles, and the
    // smallest value missing from row-to-the-left and column-above is always row ^ col.
    for row in 0..n {
        w_vec!(out, (0..n).map(|col| row ^ col).collect::<Vec<_>>());
    }
}
