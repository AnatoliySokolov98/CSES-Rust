fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let mut res = 0;
    let n = read!(it, usize);
    let mut curr = 5;
    while curr <=n {
        res += n / curr;
        curr *= 5;
    }
    wln!(out, "{}", res);
}
