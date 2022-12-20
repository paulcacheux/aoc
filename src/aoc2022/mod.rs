use crate::helpers::{run, Results, TimingData};
use crate::traits::days::*;

pub struct Aoc2022;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod grid;

pub fn run_solution_for_day(day: u32, input: &str, results: Option<Results>) -> Option<TimingData> {
    let r = results
        .as_ref()
        .and_then(|r| r.results_for_day(day as usize));

    let elapsed = match day {
        1 => run::<Aoc2022, Day1>(input, r),
        2 => run::<Aoc2022, Day2>(input, r),
        3 => run::<Aoc2022, Day3>(input, r),
        4 => run::<Aoc2022, Day4>(input, r),
        5 => run::<Aoc2022, Day5>(input, r),
        6 => run::<Aoc2022, Day6>(input, r),
        7 => run::<Aoc2022, Day7>(input, r),
        8 => run::<Aoc2022, Day8>(input, r),
        9 => run::<Aoc2022, Day9>(input, r),
        10 => run::<Aoc2022, Day10>(input, r),
        11 => run::<Aoc2022, Day11>(input, r),
        12 => run::<Aoc2022, Day12>(input, r),
        13 => run::<Aoc2022, Day13>(input, r),
        14 => run::<Aoc2022, Day14>(input, r),
        15 => run::<Aoc2022, Day15>(input, r),
        16 => run::<Aoc2022, Day16>(input, r),
        17 => run::<Aoc2022, Day17>(input, r),
        18 => run::<Aoc2022, Day18>(input, r),
        19 => run::<Aoc2022, Day19>(input, r),
        20 => run::<Aoc2022, Day20>(input, r),
        _ => return None,
    };
    Some(elapsed)
}
