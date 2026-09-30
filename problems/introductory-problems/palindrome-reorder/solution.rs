fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let s = read!(it, String).into_bytes();
    let mut res = vec![b'A'; s.len()];
    let mut l = 0;
    let mut r = s.len() - 1;
    let mut counts = [0; 26];
    for char in s {
        let index = (char - b'A') as usize;
        counts[index] += 1;
    }
    let mut odds = 0;
    for &count in &counts {
        if count % 2 == 1 {
            odds += 1;
        }
    }
    if odds > 1 {
        wln!(out, "NO SOLUTION");
        return;
    }
    let mut odd_index = 26;
    for (i, &v) in counts.iter().enumerate() {
        let c = b'A' + (i as u8);
        let half = v / 2;
        for _ in 0..half {
            res[l] = c;
            res[r] = c;
            l += 1;
            r -= 1;
        }
        if v % 2 == 1 {
            odd_index = i;
        }
    }
    if odd_index != 26 {
        let c = b'A' + (odd_index as u8);
        res[l] = c;
    }
    let res = String::from_utf8(res).unwrap();
    wln!(out, "{res}")
}
