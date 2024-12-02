# Advent of Code challenges

These are my submissions to the
[Advent of Code](https://adventofcode.com/), solved in Rust.

Please note: A lot of these solutions are a bit hacky or poorly organized. They
are not written or documented in the same way I'd write Production code, and
these solutions favor panicking and over-utilize `unwrap()`, rather than
handling errors appropriately. I haven't gone through to clean up the solutions
once I got the right answers, and there are known bugs for some solutions,
even though they work for the puzzle inputs.
Solutions haven't been optimized for performance once the right answer has been
reached.

Each solution contains tests for the sample input, as well as the final answers.
Run `cargo test` to run all tests. To run a specific day `n`, use
`cargo run --bin day_n`. To use a custom input, either replace the file in the
`inputs` directory, or modify the code to read in a different file.

For any questions about my solutions, approach, or if you discover an
undocumented bug, feel free to
[open an Issue](https://github.com/nikwithak/advent_of_code_2023/issues/new/choose).
