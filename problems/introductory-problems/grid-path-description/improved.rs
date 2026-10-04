const N: usize = 7;
// (row step, col step, letter). usize::MAX acts as -1 under wrapping_add.
const MOVES: [(usize, usize, u8); 4] = [
    (0, usize::MAX, b'L'),
    (0, 1, b'R'),
    (usize::MAX, 0, b'U'),
    (1, 0, b'D'),
];

fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let s = read!(it, String).into_bytes();
    // The 7x7 grid lives at rows/cols 1..=7 inside a border that starts out visited,
    // so neighbors never need bounds checks.
    let mut visited = [[true; N + 2]; N + 2];
    for row in 1..=N {
        for col in 1..=N {
            visited[row][col] = false;
        }
    }
    wln!(out, "{}", count(1, 1, 0, &s, &mut visited));
}

fn count(row: usize, col: usize, index: usize, s: &[u8], visited: &mut [[bool; N + 2]; N + 2]) -> u64 {
    // Reaching the bottom-left corner only counts once all 49 cells are visited.
    if (row, col) == (N, 1) {
        return (index == N * N - 1) as u64;
    }
    // Blocked on both sides of one axis but open on the other: the unvisited cells
    // are split in two, and the path can't cover both halves.
    let (left, right) = (visited[row][col - 1], visited[row][col + 1]);
    let (up, down) = (visited[row - 1][col], visited[row + 1][col]);
    if left == right && up == down && left != up {
        return 0;
    }
    visited[row][col] = true;
    let mut res = 0;
    for (dr, dc, dir) in MOVES {
        let (nr, nc) = (row.wrapping_add(dr), col.wrapping_add(dc));
        if (s[index] == b'?' || s[index] == dir) && !visited[nr][nc] {
            res += count(nr, nc, index + 1, s, visited);
        }
    }
    visited[row][col] = false;
    res
}
