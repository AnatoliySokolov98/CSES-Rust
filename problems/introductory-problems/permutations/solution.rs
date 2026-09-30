fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, usize);
    if n == 1 {
        wln!(out, "1");
        return;
    }
    if n <= 3 {
        wln!(out, "NO SOLUTION");
        return;
    }
    for i in (2..=n).step_by(2) {
        wln!(out, " {i}");
    }
    for i in (1..=n).step_by(2) {
        wln!(out, "  {i}");
    }
}
