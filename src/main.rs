use advent_of_code_traits::{days::Day1, ParseEachInput, Part1, Part2, Solution};
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

fn run<A: Solution<D>, const D: u32>(input: &str) {
    let part1_parsed_input = <A as ParseEachInput<D, Part1>>::parse_input(input);
    let part1_output = A::part1(&part1_parsed_input);
    println!("Day {}, Part 1: {}", D, part1_output);

    let part2_parsed_input = <A as ParseEachInput<D, Part2>>::parse_input(input);
    let part2_output = A::part2(&part2_parsed_input);
    println!("Day {}, Part 2: {}", D, part2_output);
}

fn run_solution_for_day(day: u32, input: &str) {
    match day {
        1 => run::<aoc::Aoc2021, Day1>(input),
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
