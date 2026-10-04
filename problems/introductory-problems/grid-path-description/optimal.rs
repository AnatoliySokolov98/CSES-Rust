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
    let forced = forced_move(row, col, visited);
    let mut res = 0;
    for (k, (dr, dc, dir)) in MOVES.into_iter().enumerate() {
        let (nr, nc) = (row.wrapping_add(dr), col.wrapping_add(dc));
        let allowed = match forced {
            Forced::Dead => false,
            Forced::Move(f) => f == k,
            Forced::Free => true,
        };
        if allowed && (s[index] == b'?' || s[index] == dir) && !visited[nr][nc] {
            res += count(nr, nc, index + 1, s, visited);
        }
    }
    visited[row][col] = false;
    res
}

enum Forced {
    Free,
    Move(usize),
    Dead,
}

// Every cell except the end is entered once and left once, so it needs two free
// neighbors. A free neighbor of (row, col) with at most one other free neighbor
// must be entered right now, while (row, col) can still be its way in. Two such
// neighbors can't both be entered next, so the path is dead.
fn forced_move(row: usize, col: usize, visited: &[[bool; N + 2]; N + 2]) -> Forced {
    let mut forced = Forced::Free;
    for (k, (dr, dc, _)) in MOVES.into_iter().enumerate() {
        let (nr, nc) = (row.wrapping_add(dr), col.wrapping_add(dc));
        if visited[nr][nc] || (nr, nc) == (N, 1) {
            continue;
        }
        let exits = MOVES
            .iter()
            .filter(|&&(a, b, _)| !visited[nr.wrapping_add(a)][nc.wrapping_add(b)])
            .count();
        if exits <= 1 {
            if let Forced::Move(_) = forced {
                return Forced::Dead;
            }
            forced = Forced::Move(k);
        }
    }
    forced
}
