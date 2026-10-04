fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let s = read!(it, String).into_bytes();
    let mut counts = [0usize; 26];
    for &c in &s {
        counts[(c - b'A') as usize] += 1;
    }
    let odd: Vec<u8> = (b'A'..=b'Z').filter(|&c| counts[(c - b'A') as usize] % 2 == 1).collect();
    if odd.len() > 1 {
        wln!(out, "NO SOLUTION");
        return;
    }
    // Left half in sorted order, the odd letter (if any) in the middle, then the mirror.
    let mut half = Vec::with_capacity(s.len() / 2);
    for (c, &count) in (b'A'..=b'Z').zip(&counts) {
        half.extend(std::iter::repeat(c).take(count / 2));
    }
    let mut res = half.clone();
    res.extend(odd);
    res.extend(half.iter().rev());
    wln!(out, "{}", String::from_utf8(res).unwrap());
}
