fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let t = read!(it, usize);
    for _ in 0..t {
        solve_once(it, out);
    }
}

fn solve_once(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let (n, a, b) = read!(it, usize, usize, usize);
    // Both players hold the same cards, so their win margins must cancel out: one
    // player cannot win rounds unless the other wins some too.
    if a + b > n || (a == 0) != (b == 0) {
        wln!(out, "NO");
        return;
    }
    // Player 1 plays card k in round k. On cards 1..=a+b, player 2 plays k shifted
    // up by a (wrapping around), which loses to player 1 exactly a times.
    // Cards above a+b are ties.
    let m = a + b;
    let second: Vec<usize> = (1..=n)
        .map(|k| if k > m { k } else if k <= b { k + a } else { k - b })
        .collect();
    wln!(out, "YES");
    w_vec!(out, (1..=n).collect::<Vec<_>>());
    w_vec!(out, second);
}
