use crate::aoc2024::Aoc2024;
use crate::traits::days::Day1;
use crate::traits::ParseInput;
use crate::traits::Solution;

impl ParseInput<Day1> for Aoc2024 {
    type Parsed = (Vec<u32>, Vec<u32>);

    fn parse_input(input: &str) -> Self::Parsed {
        let mut lefts = Vec::new();
        let mut rights = Vec::new();

        for line in input.lines() {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            let mut parts = line.split_ascii_whitespace();
            let left = parts.next().unwrap().parse().unwrap();
            let right = parts.next().unwrap().parse().unwrap();

            lefts.push(left);
            rights.push(right);
        }

        (lefts, rights)
    }
}

impl Solution<Day1> for Aoc2024 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
        let mut left = left.clone();
        left.sort();

        let mut right = right.clone();
        right.sort();

        left.into_iter()
            .zip(right)
            .map(|(l, r)| l.abs_diff(r))
            .sum()
    }

    fn part2((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
        let mut score = 0;

        for l in left {
            let count = right.iter().filter(|&r| r == l).count() as u32;
            score += l * count;
        }

        score
    }
}
