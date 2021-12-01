use crate::aoc::Aoc2021;
use advent_of_code_traits::days::Day1;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;

impl ParseInput<Day1> for Aoc2021 {
    type Parsed = Vec<u32>;

    fn parse_input(input: &str) -> Vec<u32> {
        let mut values = Vec::new();
        for line in input.lines() {
            let value = line.parse().expect("Failed to parse integer");
            values.push(value);
        }
        values
    }
}

fn count_increasing_windows(values: &[u32]) -> usize {
    let mut counter = 0;
    for w in values.windows(2) {
        let (a, b) = (w[0], w[1]);
        if a < b {
            counter += 1;
        }
    }
    counter
}

impl Solution<Day1> for Aoc2021 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Vec<u32>) -> usize {
        count_increasing_windows(input)
    }

    fn part2(input: &Vec<u32>) -> usize {
        let sums: Vec<u32> = input
            .windows(3)
            .map(|w| w.iter().copied().reduce(|a, b| a + b).unwrap())
            .collect();
        count_increasing_windows(&sums)
    }
}
