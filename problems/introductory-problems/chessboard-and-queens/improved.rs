const N: usize = 8;

fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let board: Vec<Vec<u8>> = (0..N).map(|_| read!(it, String).into_bytes()).collect();
    wln!(out, "{}", count(&board, 0, 0, 0, 0));
}

// Bit i of `cols` / `diags` / `antis` marks column i / diagonal row + col /
// anti-diagonal row - col + N - 1 as attacked. Masks are passed by value, so
// nothing has to be undone after the recursive call.
fn count(board: &[Vec<u8>], row: usize, cols: u32, diags: u32, antis: u32) -> usize {
    if row == N {
        return 1;
    }
    let mut res = 0;
    for col in 0..N {
        let (c, d, a) = (1 << col, 1 << (row + col), 1 << (row + N - 1 - col));
        if board[row][col] == b'*' || cols & c != 0 || diags & d != 0 || antis & a != 0 {
            continue;
        }
        res += count(board, row + 1, cols | c, diags | d, antis | a);
    }
    res
}
