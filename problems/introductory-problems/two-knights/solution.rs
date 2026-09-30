fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, i64);
    for i in 1..=n {
        let res = (i * i) * (i * i - 1) / 2 - (i - 1) * (i - 2) * 4;
        wln!(out, "{}", res)
    }
}
