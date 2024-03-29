use crate::aoc2021::Aoc2021;
use crate::traits::days::Day1;
use crate::traits::ParseInput;
use crate::traits::Solution;

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

impl Solution<Day1> for Aoc2021 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Vec<u32>) -> usize {
        let mut counter = 0;
        for w in input.windows(2) {
            let (a, b) = (w[0], w[1]);
            if a < b {
                counter += 1;
            }
        }
        counter
    }

    fn part2(input: &Vec<u32>) -> usize {
        let mut counter = 0;
        for w in input.windows(4) {
            if w[0] < w[3] {
                counter += 1;
            }
        }
        counter
    }
}
