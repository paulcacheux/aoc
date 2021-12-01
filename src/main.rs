use advent_of_code_traits::{days::Day1, Solution};
use clap::Parser;

mod aoc;
mod day1;

/// A subcommand for controlling testing
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

fn call_solution_for_day(day: u32) -> for<'r> fn(&'r str) {
    match day {
        1 => <aoc::Aoc2021 as Solution<Day1>>::run,
        _ => unimplemented!("no solution available for that day"),
    }
}

fn main() {
    let opts = Options::parse();

    let input_path = if opts.test {
        format!("./inputs/day{}_test.txt", opts.day)
    } else {
        format!("./inputs/day{}.txt", opts.day)
    };
    let input = std::fs::read_to_string(input_path).expect("failed to read input");

    call_solution_for_day(opts.day)(&input);
}
