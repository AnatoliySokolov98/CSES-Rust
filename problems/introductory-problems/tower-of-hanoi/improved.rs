fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, u32);
    // The minimum number of moves is known up front, so moves can print as they happen.
    wln!(out, "{}", (1u32 << n) - 1);
    hanoi(n, 1, 3, 2, out);
}

fn hanoi(n: u32, from: u8, to: u8, via: u8, out: &mut impl std::io::Write) {
    if n == 0 {
        return;
    }
    hanoi(n - 1, from, via, to, out);
    wln!(out, "{from} {to}");
    hanoi(n - 1, via, to, from, out);
}
