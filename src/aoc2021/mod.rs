use crate::helpers::{run, Results};
use advent_of_code_traits::days::*;

pub struct Aoc2021;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub fn run_solution_for_day(day: u32, input: &str, results: Option<Results>) -> bool {
    let r = results
        .as_ref()
        .and_then(|r| r.results_for_day(day as usize));

    match day {
        1 => run::<Aoc2021, Day1>(input, r),
        2 => run::<Aoc2021, Day2>(input, r),
        3 => run::<Aoc2021, Day3>(input, r),
        4 => run::<Aoc2021, Day4>(input, r),
        5 => run::<Aoc2021, Day5>(input, r),
        6 => run::<Aoc2021, Day6>(input, r),
        7 => run::<Aoc2021, Day7>(input, r),
        8 => run::<Aoc2021, Day8>(input, r),
        9 => run::<Aoc2021, Day9>(input, r),
        10 => run::<Aoc2021, Day10>(input, r),
        11 => run::<Aoc2021, Day11>(input, r),
        _ => return false,
    }
    true
}
