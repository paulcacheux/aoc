use advent_of_code_traits::{days::*, ParseEachInput, Part1, Part2, Solution};
use clap::Parser;

mod aoc;
mod day1;
mod day2;
mod day3;

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

macro_rules! inner_run {
    ($P:tt, $F:expr, $input:expr) => {{
        let parsed_input = <A as ParseEachInput<D, $P>>::parse_input($input);
        let start = std::time::Instant::now();
        let output = $F(&parsed_input);
        let elapsed = start.elapsed();
        println!("Day {}, Part {}: {}, in {:?}", D, $P, output, elapsed,);
    }};
}

fn run<A: Solution<D>, const D: u32>(input: &str) {
    inner_run!(Part1, A::part1, input);
    inner_run!(Part2, A::part2, input);
}

fn run_solution_for_day(day: u32, input: &str) {
    match day {
        1 => run::<aoc::Aoc2021, Day1>(input),
        2 => run::<aoc::Aoc2021, Day2>(input),
        3 => run::<aoc::Aoc2021, Day3>(input),
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

    run_solution_for_day(opts.day, &input);
}
