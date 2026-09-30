fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, usize);
    let mut res = vec![0,1];
    for i in 1..n {
        let length = res.len();
        for j in (0..length).rev() {
            res.push((1 << i) | res[j]);
        }
    }

    for item in res {
        for i in 0..n {
            let val = (1<<(n - 1 - i)) & item;
            if val == 0  {
                w!(out, "0");
            } else {
                w!(out, "1");
            }
        }
        wln!(out, "");
    }
}
