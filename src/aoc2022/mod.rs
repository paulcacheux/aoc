use std::time::Duration;

use crate::helpers::{run, Results};
use advent_of_code_traits::days::*;

pub struct Aoc2022;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

pub fn run_solution_for_day(day: u32, input: &str, results: Option<Results>) -> Option<Duration> {
    let r = results
        .as_ref()
        .and_then(|r| r.results_for_day(day as usize));

    let elapsed = match day {
        1 => run::<Aoc2022, Day1>(input, r),
        2 => run::<Aoc2022, Day2>(input, r),
        3 => run::<Aoc2022, Day3>(input, r),
        4 => run::<Aoc2022, Day4>(input, r),
        5 => run::<Aoc2022, Day5>(input, r),
        _ => return None,
    };
    Some(elapsed)
}
