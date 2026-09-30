fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, usize);
    let mut nums: Vec<i64> = (0..n).map(|_|read!(it, i64)).collect();
    let mut res = 0;
    for i in 1..n {
        if nums[i] >= nums[i - 1] {
            continue;
        }
        res += nums[i - 1] - nums[i];
        nums[i] = nums[i - 1];
    }
    wln!(out, "{res}");
}
