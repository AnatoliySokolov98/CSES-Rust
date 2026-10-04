fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let s = read!(it, String).into_bytes();
    let mut counts = [0usize; 26];
    for &c in &s {
        counts[(c - b'A') as usize] += 1;
    }
    // A letter can fill at most every other slot.
    if 2 * counts.iter().max().unwrap() > s.len() + 1 {
        wln!(out, "-1");
        return;
    }
    let mut res = Vec::with_capacity(s.len());
    let mut prev = None;
    for left in (1..=s.len()).rev() {
        // A letter holding more than half of the remaining slots must go now, or
        // there is no room left to separate its copies. Otherwise the smallest
        // letter that differs from the previous one is safe.
        let pick = (0..26)
            .find(|&i| 2 * counts[i] > left)
            .or_else(|| (0..26).find(|&i| counts[i] > 0 && Some(i) != prev))
            .unwrap();
        counts[pick] -= 1;
        res.push(b'A' + pick as u8);
        prev = Some(pick);
    }
    wln!(out, "{}", String::from_utf8(res).unwrap());
}
