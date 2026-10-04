fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, usize);
    // Each cell's value is the mex of at most 2(n - 1) earlier values, so it stays
    // below 2n and a boolean table can replace the hash sets.
    let mut in_row = vec![vec![false; 2 * n]; n];
    let mut in_col = vec![vec![false; 2 * n]; n];
    let mut grid = vec![vec![0; n]; n];
    for row in 0..n {
        for col in 0..n {
            let val = (0..).find(|&v| !in_row[row][v] && !in_col[col][v]).unwrap();
            in_row[row][val] = true;
            in_col[col][val] = true;
            grid[row][col] = val;
        }
    }
    for row in &grid {
        w_vec!(out, row);
    }
}
