use std::collections::HashSet;

fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, usize);
    let mut board = vec![vec![0;n];n];
    let mut rows = vec![HashSet::new(); n];
    let mut cols = vec![HashSet::new(); n];
    for row in 0..n {
        for col in 0..n {
            let mut val = 0;
            while rows[row].contains(&val) || cols[col].contains(&val) {
                val += 1;
            }
            rows[row].insert(val);
            cols[col].insert(val);
            board[row][col] = val;
        }
    }
    for row in board{
        w_vec!(out, row);
    }
}
