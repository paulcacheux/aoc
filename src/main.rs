use advent_of_code_traits::{days::Day1, Solution};

mod aoc;
mod day1;

fn main() {
    let input = std::fs::read_to_string("./inputs/day1.txt").expect("failed to read input");
    <aoc::Aoc2021 as Solution<Day1>>::run(&input);
}
