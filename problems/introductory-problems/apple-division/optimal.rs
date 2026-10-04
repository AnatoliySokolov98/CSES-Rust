fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, usize);
    let weights: Vec<i64> = (0..n).map(|_| read!(it, i64)).collect();
    let total: i64 = weights.iter().sum();
    wln!(out, "{}", best_split(&weights, 0, total));
}

// Each apple goes into the first group or not. Carrying the running sum down the
// recursion costs O(1) per subset, O(2^n) total, instead of re-summing every mask.
fn best_split(weights: &[i64], group: i64, total: i64) -> i64 {
    match weights.split_first() {
        None => (total - 2 * group).abs(),
        Some((&w, rest)) => best_split(rest, group + w, total).min(best_split(rest, group, total)),
    }
}
