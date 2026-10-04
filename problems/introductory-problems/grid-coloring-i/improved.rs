fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let (rows, cols) = read!(it, usize, usize);
    let mut grid: Vec<Vec<u8>> = (0..rows).map(|_| read!(it, String).into_bytes()).collect();
    for row in 0..rows {
        for col in 0..cols {
            let original = grid[row][col];
            let up = if row > 0 { grid[row - 1][col] } else { 0 };
            let left = if col > 0 { grid[row][col - 1] } else { 0 };
            // At most three letters are ruled out, so one of the four is always free
            // and the answer is never IMPOSSIBLE.
            grid[row][col] = *b"ABCD"
                .iter()
                .find(|&&c| c != original && c != up && c != left)
                .unwrap();
        }
    }
    for row in &grid {
        wln!(out, "{}", std::str::from_utf8(row).unwrap());
    }
}
