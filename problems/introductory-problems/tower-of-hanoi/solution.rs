fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, usize);
    let mut res = Vec::new();
    hanoi(n, 1, 3, 2, &mut res);
    wln!(out, "{}", res.len());
    for (x, y) in res {
        wln!(out, "{} {}", x, y);
    }
}

fn hanoi(n: usize, first: usize, last: usize, middle: usize, res: &mut Vec<(usize, usize)>) {
    if n == 1 {
        res.push((first, last));
        return;
    }
    hanoi(n - 1, first, middle, last, res);
    hanoi(1, first, last, middle, res);
    hanoi(n - 1, middle, last, first, res);
}
