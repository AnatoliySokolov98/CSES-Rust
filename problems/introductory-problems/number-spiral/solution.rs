fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, usize);
    for _ in 0..n {
        solve_once(it, out);
    }
}

fn solve_once(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let (mut row, mut col) = read!(it, usize, usize);
    row -= 1;
    col -= 1;
    let big = row.max(col);
    let square = (big + 1) * (big + 1) - big;
    if row < col {
        if col % 2 == 0 {
            wln!(out, "{}", square + (col - row));
        } else {
            wln!(out, "{}", square - (col - row));
        }
    } else {
        if row % 2 == 0 {
            wln!(out, "{}", square - (row - col));
        } else {
            wln!(out, "{}", square + (row - col));
        }
    }
}
