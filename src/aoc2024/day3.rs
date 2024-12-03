use regex::Regex;

use crate::aoc2024::Aoc2024;
use crate::traits::days::Day3;
use crate::traits::ParseInput;
use crate::traits::Solution;

impl ParseInput<Day3> for Aoc2024 {
    type Parsed = String;

    fn parse_input(input: &str) -> Self::Parsed {
        input.into()
    }
}

impl Solution<Day3> for Aoc2024 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &String) -> u32 {
        compute_part_sum(input)
    }

    fn part2(input: &String) -> u32 {
        let mut input = input.as_str();
        let mut enabled = true;
        let mut sum = 0;

        while !input.is_empty() {
            if enabled {
                let end = input.find("don't()").unwrap_or(input.len());
                let part = &input[..end];
                sum += compute_part_sum(part);
                input = &input[end..];
            } else {
                let end = input.find("do()").unwrap_or(input.len());
                input = &input[end..];
            }
            enabled = !enabled;
        }
        sum
    }
}

fn compute_part_sum(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let a = cap[1].parse::<u32>().unwrap();
            let b = cap[2].parse::<u32>().unwrap();
            a * b
        })
        .sum()
}
