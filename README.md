# Advent of Code 2016, faern's solutions

Here are my solutions built in Rust. The folder structure is as follows:

* `aoc/` - The CLI for the problem solver. Read below how to use it.
* `dayX/` - The crate containing the solution for dayX.
* `base/` - Some shared interfaces and utilities between the solutions and the
  CLI. Such as the enum abstraction for a `Part` denoting if we are solving
  part one or two on a given day.

## How to use

### Compile

* Get a working Rust environment. see https://rustup.rs/. Should work fine with
  both stable and nightly.
* Cd into `aoc/`.
* Run `cargo build --release` to compile with optimizations.

### Run

* Run `./target/release/aoc --help` for help
  * The flag `--day X` will select problem for day `X`
  * The flag `--part Y` will select part `Y`, must be 1 or 2
  * The flag `--input <path>` selects which problem input file to read from

#### Example usage

`./target/release/aoc --day 1 --part 2 --input inputs/day1.txt`
