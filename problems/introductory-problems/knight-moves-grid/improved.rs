use std::collections::VecDeque;

fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, usize);
    let mut board = vec![vec![usize::MAX;n];n];
    let directions = [(2,1),(-2,1),(2,-1), (-2,-1), (1,2), (1,-2), (-1,2), (-1,-2)];
    let mut bfs = VecDeque::new();
    bfs.push_back((0,0));
    board[0][0] = 0;
    while let Some((row, col)) = bfs.pop_front() {
        for &(x, y) in &directions {
            let nx = row as i32 + x;
            let ny = col as i32 + y;
            if nx < 0 || ny < 0  {
                continue;
            }
            let xs = nx as usize;
            let ys = ny as usize;
            if xs < n && ys < n && board[xs][ys] == usize::MAX {
                board[xs][ys] = board[row][col] + 1;
                bfs.push_back((xs, ys));
            }
        }
    }
    for row in board {
        w_vec!(out, row);
    }
}
