use crate::aoc2019::Aoc2019;
use advent_of_code_traits::days::Day1;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;

impl ParseInput<Day1> for Aoc2019 {
    type Parsed = Vec<u32>;

    fn parse_input(input: &str) -> Vec<u32> {
        input.lines().map(|line| line.parse().unwrap()).collect()
    }
}

fn count_fuel_part1(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

fn count_fuel_part2(mass: u32) -> u32 {
    let mut current_mass = mass;
    let mut total_fuel = 0;
    while current_mass != 0 {
        let fuel = count_fuel_part1(current_mass);
        total_fuel += fuel;
        current_mass = fuel;
    }
    total_fuel
}

impl Solution<Day1> for Aoc2019 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Vec<u32>) -> u32 {
        input.iter().copied().map(count_fuel_part1).sum()
    }

    fn part2(input: &Vec<u32>) -> u32 {
        input.iter().copied().map(count_fuel_part2).sum()
    }
}
