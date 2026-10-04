fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    // Values climb past 2^32 (n < 10^6 peaks near 5.7 * 10^10), so use u64, not usize.
    let mut n = read!(it, u64);
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
