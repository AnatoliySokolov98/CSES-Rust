fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, usize);
    for _ in 0..n {
        solve_once(it, out);
    }
}

fn solve_once(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let (mut n, a, b) = read!(it, usize, usize, usize);
    if a + b > n {
        wln!(out, "NO");
        return;
    }
    if (a > 0 && b == 0) || (b > 0 && a == 0) {
        wln!(out, "NO");
        return;
    }
    let ties = n - a - b;
    let mut a_moves = Vec::new();
    let mut b_moves = Vec::new();
    for _ in 0..ties {
        a_moves.push(n);
        b_moves.push(n);
        n -= 1;
    }
    for i in 0..a {
        a_moves.push(n - i);
        b_moves.push(a - i);
    }
    for i in 0..b {
        b_moves.push(n - i);
        a_moves.push(b - i);
    }
    wln!(out, "YES");
    w_vec!(out, a_moves);
    w_vec!(out, b_moves);
}
