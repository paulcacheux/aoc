use crate::aoc::Aoc2021;
use advent_of_code_traits::days::Day7;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;

impl ParseInput<Day7> for Aoc2021 {
    type Parsed = Vec<u32>;

    fn parse_input(input: &str) -> Vec<u32> {
        input
            .trim()
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect()
    }
}

fn count_fuel_part1(input: &[u32], target: u32) -> u32 {
    input
        .iter()
        .map(|&i| if i <= target { target - i } else { i - target })
        .sum()
}

fn count_fuel_part2(input: &[u32], target: u32) -> u32 {
    input
        .iter()
        .map(|&i| if i <= target { target - i } else { i - target })
        .map(|n| (n * (n + 1)) / 2)
        .sum()
}

fn common_search(input: &[u32], fuel_counter: fn(&[u32], u32) -> u32) -> u32 {
    let mut input = input.to_owned();
    input.sort();

    let mut value = input[input.len() / 2];
    loop {
        let previous_value = fuel_counter(&input, value - 1);
        let pos_value = fuel_counter(&input, value);
        let next_value = fuel_counter(&input, value + 1);

        if pos_value <= previous_value && pos_value <= next_value {
            return pos_value;
        }

        if previous_value < pos_value {
            value -= 1;
        } else if next_value < pos_value {
            value += 1;
        }
    }
}

impl Solution<Day7> for Aoc2021 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Vec<u32>) -> u32 {
        common_search(&input, count_fuel_part1)
    }

    fn part2(input: &Vec<u32>) -> u32 {
        common_search(&input, count_fuel_part2)
    }
}
