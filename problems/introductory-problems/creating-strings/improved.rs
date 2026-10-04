fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let mut s = read!(it, String).into_bytes();
    // Sorting first makes the backtracking emit strings in lexicographic order,
    // and puts equal letters next to each other so duplicates can be skipped.
    s.sort_unstable();
    let mut used = vec![false; s.len()];
    let mut curr = Vec::with_capacity(s.len());
    let mut res = Vec::new();
    backtrack(&s, &mut used, &mut curr, &mut res);
    wln!(out, "{}", res.len());
    for word in &res {
        wln!(out, "{word}");
    }
}

fn backtrack(s: &[u8], used: &mut [bool], curr: &mut Vec<u8>, res: &mut Vec<String>) {
    if curr.len() == s.len() {
        res.push(String::from_utf8(curr.clone()).unwrap());
        return;
    }
    for i in 0..s.len() {
        // Copies of a letter are interchangeable: take them in order, so a copy is
        // only placed once the copy before it is already in use.
        if used[i] || (i > 0 && s[i] == s[i - 1] && !used[i - 1]) {
            continue;
        }
        used[i] = true;
        curr.push(s[i]);
        backtrack(s, used, curr, res);
        curr.pop();
        used[i] = false;
    }
}
