fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let q = read!(it, usize);
    for _ in 0..q {
        solve_once(it, out);
    }
}

fn solve_once(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    // 0-indexed position in the digit string.
    let mut k = read!(it, u64) - 1;
    // Skip whole blocks of same-length numbers: 9 one-digit, 90 two-digit, ...
    let mut digits = 1;
    let mut count = 9;
    let mut first = 1;
    while k >= count * digits {
        k -= count * digits;
        digits += 1;
        count *= 10;
        first *= 10;
    }
    let number = first + k / digits;
    let digit = number.to_string().as_bytes()[(k % digits) as usize] as char;
    wln!(out, "{digit}");
}
