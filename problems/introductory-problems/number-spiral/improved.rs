fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let t = read!(it, usize);
    for _ in 0..t {
        solve_once(it, out);
    }
}

fn solve_once(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let (y, x) = read!(it, i64, i64);
    // Cell (y, x) sits on layer k, whose corner (k, k) holds k^2 - k + 1.
    let k = y.max(x);
    let corner = k * k - k + 1;
    // Odd layers count up along the row toward the corner and keep going down the
    // column; even layers run the other way.
    let value = if k % 2 == 1 { corner + x - y } else { corner + y - x };
    wln!(out, "{value}");
}
