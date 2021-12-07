use clap::Parser;

mod aoc2021;
mod helpers;

use helpers::Results;

#[derive(Parser)]
#[clap(version = "1.0", author = "Paul C. <paulcacheux@gmail.com>")]
struct Options {
    /// Use test input
    #[clap(long)]
    test: bool,
    /// Advent day
    #[clap(long)]
    day: u32,
}

fn main() {
    let opts = Options::parse();

    let (input_path, results) = if opts.test {
        (format!("./inputs/2021/day{}_test.txt", opts.day), None)
    } else {
        (
            format!("./inputs/2021/day{}.txt", opts.day),
            Some(Results::parse().expect("Failed to parse results")),
        )
    };
    let input = std::fs::read_to_string(input_path).expect("failed to read input");

    aoc2021::run_solution_for_day(opts.day, &input, results);
}
