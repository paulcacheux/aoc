use std::fs::File;

use advent_of_code_traits::{days::*, ParseEachInput, Part1, Part2, Solution};
use clap::Parser;
use serde::Deserialize;

mod aoc;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

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
    ($P:tt, $F:expr, $input:expr, $expected:expr) => {{
        let parsed_input = <A as ParseEachInput<D, $P>>::parse_input($input);
        let start = std::time::Instant::now();
        let output = $F(&parsed_input);
        let elapsed = start.elapsed();
        println!("Day {}, Part {}: {}, in {:?}", D, $P, output, elapsed,);
        if let Some(expected) = $expected {
            assert_eq!(expected, output.to_string());
        }
    }};
}

fn run<A: Solution<D>, const D: u32>(input: &str, result: Option<&DayResult>) {
    inner_run!(Part1, A::part1, input, result.map(|r| r.part1.as_str()));
    inner_run!(Part2, A::part2, input, result.map(|r| r.part2.as_str()));
}

fn run_solution_for_day(day: u32, input: &str, results: Option<Results>) {
    let r = results
        .as_ref()
        .and_then(|r| r.results_for_day(day as usize));

    match day {
        1 => run::<aoc::Aoc2021, Day1>(input, r),
        2 => run::<aoc::Aoc2021, Day2>(input, r),
        3 => run::<aoc::Aoc2021, Day3>(input, r),
        4 => run::<aoc::Aoc2021, Day4>(input, r),
        5 => run::<aoc::Aoc2021, Day5>(input, r),
        6 => run::<aoc::Aoc2021, Day6>(input, r),
        _ => unimplemented!("no solution available for that day"),
    }
}

struct Results {
    days: Vec<DayResult>,
}

impl Results {
    fn parse() -> Result<Self, Box<dyn std::error::Error>> {
        let f = File::open("./results.json")?;
        let days: Vec<DayResult> = serde_json::from_reader(f)?;

        Ok(Self { days })
    }

    fn results_for_day(&self, day: usize) -> Option<&DayResult> {
        for r in &self.days {
            if r.day == day {
                return Some(r);
            }
        }
        None
    }
}

#[derive(Debug, Deserialize)]
struct DayResult {
    day: usize,
    part1: String,
    part2: String,
}

fn main() {
    let opts = Options::parse();

    let (input_path, results) = if opts.test {
        (format!("./inputs/day{}_test.txt", opts.day), None)
    } else {
        (
            format!("./inputs/day{}.txt", opts.day),
            Some(Results::parse().expect("Failed to parse results")),
        )
    };
    let input = std::fs::read_to_string(input_path).expect("failed to read input");

    run_solution_for_day(opts.day, &input, results);
}
