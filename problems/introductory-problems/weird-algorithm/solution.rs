fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let mut n = read!(it, usize);
    while n != 1 {
        wln!(out, "{n}");
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
    }

    wln!(out, "1");
}
