fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let s = read!(it, String).into_bytes();
    let mut res = 1;
    let mut curr = 1;
    for i in 1..s.len() {
        if s[i] != s[i - 1] {
            curr = 0;
        }
        curr += 1;
        res = res.max(curr);
    }
    wln!(out, "{res}");
}
