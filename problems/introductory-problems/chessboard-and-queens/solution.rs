fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let mut board: Vec<Vec<char>> = Vec::new();
    for _ in 0..8 {
        let row = read!(it, String).chars().collect();
        board.push(row);
    }
    let mut cols = vec![false; 8];
    let mut left_diag = vec![false; 16];
    let mut right_diag = vec![false; 16];
    let res = backtrack(0, &mut cols, &mut left_diag, &mut right_diag, &board);
    wln!(out, "{}", res);
}

fn backtrack(
    row: usize,
    cols: &mut Vec<bool>,
    left_diag: &mut Vec<bool>,
    right_diag: &mut Vec<bool>,
    board: &Vec<Vec<char>>,
) -> usize {
    if row == 8 {
        return 1;
    }
    let mut res = 0;
    for col in 0..8 {
        let ld = 8 + row - col;
        let rd = col + row;
        if board[row][col] == '*' || cols[col] || left_diag[ld] || right_diag[rd] {
            continue;
        }
        cols[col] = true;
        left_diag[ld] = true;
        right_diag[rd] = true;
        res += backtrack(row + 1, cols, left_diag, right_diag, board);
        cols[col] = false;
        left_diag[ld] = false;
        right_diag[rd] = false;
    }
    res
}
