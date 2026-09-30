use std::collections::HashSet;

fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let s:  Vec<char> = read!(it, String).chars().collect();
    let mut res = HashSet::new();
    let mut curr = String::new();
    let mut used = vec![false; s.len()];
    backtrack(&s, &mut used, &mut curr, &mut res);
    let mut res: Vec<String> = res.into_iter().collect();
    res.sort();
    wln!(out, "{}", res.len());
    for item in res {
        wln!(out, "{}", item);
    }
}

fn backtrack(s: &Vec<char>, used: &mut Vec<bool>, curr: &mut String, res: &mut HashSet<String>) {
    if curr.len() == s.len() {
        res.insert(curr.clone());
        return;
    }
    for i in 0..s.len() {
        if !used[i] {
            curr.push(s[i]);
            used[i] = true;
            backtrack(s, used, curr, res);
            curr.pop();
            used[i] = false;
        }
    }
}
