fn main() {
    use std::io::{self, BufWriter, Write};

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut input = stdin.lock();
    let mut out = BufWriter::new(stdout.lock());
    solve(&mut input, &mut out);
    out.flush().expect("Failed to flush output");
}
