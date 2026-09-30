fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, usize);
    let total = (1..n).map(|_|read!(it, usize)).sum::<usize>();
    wln!(out, "{}", n * (n + 1) / 2 - total);
}
