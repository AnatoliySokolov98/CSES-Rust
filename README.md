# CSES Rust solutions

All 400 problems across 18 categories from [CSES](https://cses.fi/problemset/), with public sample inputs and outputs. Requires Rust (`rustc`, Cargo) and Python 3; no Python packages are needed.

```sh
./cses list
./cses run weird-algorithm                  # reads the problem's input.txt
./cses run 1068 --input input.txt            # custom path relative to your shell
./cses run 1068 --input -                    # read stdin
./cses test weird-algorithm                  # public samples + your saved tests
./cses submit weird-algorithm                # generate and compile; never uploads
```

Edit `problems/<category>/<name>/solution.rs`. Each file defines:

```rust
fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let (a, b) = read!(it, usize, usize);
    wln!(out, "{}", a + b);
}
```

Keep helper functions, types, and imports in the solution file. Shared macros live in `src/macros.rs`; shared `main` lives in `src/runner.rs`. Main reads all stdin, calls `solve`, and flushes buffered, locked stdout. Read one type for a single value, or multiple types for a tuple. Use `w!` / `wln!` for formatted output, `w_vec!(out, values)` for a 1D vector. For a 2D vector, loop over its rows in your solution and call `w_vec!(out, row)` for each row. Writes panic on errors internally.

Each command assembles the same standalone source in `target/submissions/<name>.rs` and compiles it with `rustc --edition=2021 -O`. Submit that generated file to CSES. It contains the macros, main, and exactly one solution with no external modules or includes. Do not edit generated files. Solutions must use the standard library and syntax supported by the judge's Rust compiler.

Each problem has an empty scratch `input.txt` (Weird Algorithm preserves the existing input), a task link in `README.md`, and `tests/sample-N.in` / `sample-N.out`. Add your own matching `.in` / `.out` pairs. The test command ignores whitespace differences, reports mismatches and runtime failures, and returns a nonzero status on failure. Execution defaults to a five-second timeout; override with `--timeout 10`. This is a sample checker, not the CSES judge: constructive problems can have multiple valid answers that differ from the saved output.

Unsolved files contain only a `solve` stub with `todo!()`, which fails explicitly when run. `cargo run < input.txt` still runs Weird Algorithm for convenience; `cargo test --bin cses` checks shared macros. Use `./cses` to select other problems.

To add a problem or category, create its folder with `solution.rs`, `input.txt`, and test pairs, then add its ID, name, category, path, and order to `problems/index.json`. Run `./cses sync` after adding index entries to create their Cargo targets. The commands discover problems from that index.

Rust-analyzer discovers every solution through the small Cargo entry points in `src/bin/`. These include the shared macros, shared runner, and corresponding solution; edit the original solution files. The wrappers are tracked source files, so no generation step is needed after cloning. If an already-open editor still shows an unlinked-file warning, reload its Cargo workspace.

```sh
cargo check --all-targets
cargo run --bin missing-number < problems/introductory-problems/missing-number/input.txt
./cses sync                    # refresh Cargo entry points after adding problems
```

`cargo run` without `--bin` retains the original Weird Algorithm default. Submission generation continues to assemble standalone files directly from the shared sources and solution, without including the Cargo wrappers.

Six tasks in Interactive Problems use `src/interactive_runner.rs` and a `solve(input: &mut impl std::io::BufRead, out: &mut impl std::io::Write)` signature. Read replies incrementally (for example with `input.read_line(...)`) and call `out.flush()` after each query; the normal `read!` macro still needs a token iterator. Their examples are saved as `tests/example-transcript.txt`, not misleading input/output pairs. `./cses test` rejects interactive tasks; run with `--input -` and a live interactor, or generate a standalone file with `submit`. Entries with `"interactive": true` select this runner automatically.
