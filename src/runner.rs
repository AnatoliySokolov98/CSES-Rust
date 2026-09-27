fn main() {
    use std::io::{self, BufWriter, Read, Write};

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut input = String::new();
    stdin
        .lock()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let mut it = input.split_whitespace();
    let mut out = BufWriter::new(stdout.lock());

    solve(&mut it, &mut out);
    out.flush().expect("Failed to flush output");
}
