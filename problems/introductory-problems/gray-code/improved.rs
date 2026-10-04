fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, usize);
    // Reflect: the codes for n bits are the codes for n - 1 bits, then the same
    // codes in reverse order with bit n - 1 set.
    let mut codes: Vec<u32> = vec![0];
    for bit in 0..n {
        for j in (0..codes.len()).rev() {
            codes.push(codes[j] | 1 << bit);
        }
    }
    for code in codes {
        wln!(out, "{code:0n$b}");
    }
}
