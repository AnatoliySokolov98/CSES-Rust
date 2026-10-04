fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let s: Vec<char> = read!(it, String).chars().collect();
    let mut grid: [[bool; 7]; 7] = [[false; 7]; 7];
    let res = backtrack(0, 0, &s, &mut grid, 0);
    wln!(out, "{}", res);
}

fn backtrack(row: i32, col: i32, s: &Vec<char>, visited: &mut [[bool; 7]], index: usize) -> i32 {
    if row == 6 && col == 0 {
        if index == 48 {
            return 1;
        } else {
            return 0;
        }
    }

    let mut res = 0;
    let char = s[index];
    let mut x_blocked = 0;
    let mut y_blocked = 0;
    if col == 0 || visited[row as usize][(col - 1) as usize] {
        x_blocked += 1;
    }
    if col == 6 || visited[row as usize][(col + 1) as usize] {
        x_blocked += 1;
    }
    if row == 0 || visited[(row - 1) as usize][col as usize] {
        y_blocked += 1;
    }
    if row == 6 || visited[(row + 1) as usize][col as usize] {
        y_blocked += 1;
    }
    if x_blocked == 0 && y_blocked == 2 {
        return 0;
    }
    if y_blocked == 0 && x_blocked == 2 {
        return 0;
    }
    visited[row as usize][col as usize] = true;

    if char == '?' || char == 'L' {
        let nr = row;
        let nc = col - 1;
        if nr >= 0 && nr < 7 && nc >= 0 && nc < 7 && !visited[nr as usize][nc as usize] {
            res += backtrack(nr, nc, s, visited, index + 1);
        }
    }
    if char == '?' || char == 'R' {
        let nr = row;
        let nc = col + 1;
        if nr >= 0 && nr < 7 && nc >= 0 && nc < 7 && !visited[nr as usize][nc as usize] {
            res += backtrack(nr, nc, s, visited, index + 1);
        }
    }
    if char == '?' || char == 'U' {
        let nr = row - 1;
        let nc = col;
        if nr >= 0 && nr < 7 && nc >= 0 && nc < 7 && !visited[nr as usize][nc as usize] {
            res += backtrack(nr, nc, s, visited, index + 1);
        }
    }
    if char == '?' || char == 'D' {
        let nr = row + 1;
        let nc = col;
        if nr >= 0 && nr < 7 && nc >= 0 && nc < 7 && !visited[nr as usize][nc as usize] {
            res += backtrack(nr, nc, s, visited, index + 1);
        }
    }
    visited[row as usize][col as usize] = false;
    return res;
}
