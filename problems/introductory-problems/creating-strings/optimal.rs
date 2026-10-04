fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let mut s = read!(it, String).into_bytes();
    s.sort_unstable();
    let mut res = vec![String::from_utf8(s.clone()).unwrap()];
    while next_permutation(&mut s) {
        res.push(String::from_utf8(s.clone()).unwrap());
    }
    wln!(out, "{}", res.len());
    for word in &res {
        wln!(out, "{word}");
    }
}

// Rearranges s into the next larger permutation; returns false once s is the largest.
// Equal letters are handled naturally, so every distinct string appears exactly once.
fn next_permutation(s: &mut [u8]) -> bool {
    // Find the rightmost i with s[i] < s[i + 1]; everything after i is non-increasing.
    let Some(i) = (0..s.len().saturating_sub(1)).rev().find(|&i| s[i] < s[i + 1]) else {
        return false;
    };
    // Swap s[i] with the smallest larger letter to its right, then make the suffix smallest.
    let j = (i + 1..s.len()).rev().find(|&j| s[j] > s[i]).unwrap();
    s.swap(i, j);
    s[i + 1..].reverse();
    true
}
