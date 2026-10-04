fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, usize);
    // The i-th reflected Gray code is i ^ (i >> 1): consecutive values differ in the
    // lowest bit that changes when counting from i to i + 1. No list to build.
    for i in 0u32..1 << n {
        wln!(out, "{:0width$b}", i ^ (i >> 1), width = n);
    }
}
