fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let (rows, cols) = read!(it, usize, usize);
    let mut grid = Vec::new();
    let choices = ['A', 'B', 'C', 'D'];
    for _ in 0..rows {
        let chars: Vec<char> = read!(it, String).chars().collect();
        grid.push(chars);
    }
    for row in 0..rows {
        for col in 0..cols {
            let mut skipped = Vec::new();
            if row > 0 {
                skipped.push(grid[row - 1][col]);
            }
            if col > 0 {
                skipped.push(grid[row][col - 1]);
            }
            skipped.push(grid[row][col]);
            for &item in &choices {
                if !skipped.contains(&item) {
                    grid[row][col] = item;
                    break;
                }
            }
        }
    }
    for row in grid {
        let item: String = row.into_iter().collect();
        wln!(out, "{}", item);
    }
}
