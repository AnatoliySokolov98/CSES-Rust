fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, usize);
    
    let total: usize = n * (n + 1) / 2;
    if total % 2 == 1 {
        wln!(out, "NO");
        return;
    }
    let mut nums1 = Vec::new();
    let mut nums2 = Vec::new();
    let mut half = total / 2;
    for i in (1..=n).rev() {
        if i <= half {
            nums1.push(i);
            half -= i;
        } else {
            nums2.push(i);
        }
    }
    wln!(out, "YES");
    wln!(out, "{}", nums1.len());
    w_vec!(out, &nums1);
    wln!(out, "{}", nums2.len());
    w_vec!(out, &nums2);
}
