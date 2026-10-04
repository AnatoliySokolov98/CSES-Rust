fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, usize);
    for _ in 0..n {
        solve_once(it, out);
    }
}

fn solve_once(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let mut k = read!(it, usize);
    k -= 1;
    let mut digits = 1;
    let mut nums = 9;
    let mut start = 1;
    while k >= nums * digits {
        k -= nums * digits;
        digits += 1;
        nums *= 10;
        start *= 10;
    }
    let mut res = start + k / digits;
    let loc = (digits - 1) - k % digits;
    for _ in 0..loc {
        res /= 10;
    }
    wln!(out,"{}", res %10);
}