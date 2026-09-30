fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, usize);
    let nums: Vec<i64> = (0..n).map(|_|read!(it, i64)).collect();
    let masks: usize = (1 << n) - 1;
    let mut res = i64::MAX;
    for mask in 0..masks {
        let mut first = 0;
        let mut second = 0;
        for i in 0..n {
            if ((1<<i) & mask) != 0 {
                first += nums[i];
            } else {
                second += nums[i];
            }
        }
        let diff = (first - second).abs();
        res = res.min(diff);
    }
    wln!(out, "{}", res);
}
