use crate::aoc2022::Aoc2022;
use advent_of_code_traits::days::Day3;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;
use itertools::Itertools;
use std::collections::HashSet;

impl ParseInput<Day3> for Aoc2022 {
    type Parsed = Vec<Vec<u8>>;

    fn parse_input(input: &str) -> Vec<Vec<u8>> {
        input
            .lines()
            .map(|line| line.trim().bytes().collect())
            .collect()
    }
}

fn priority(c: u8) -> u8 {
    match c {
        b'a'..=b'z' => c - b'a' + 1,
        b'A'..=b'Z' => c - b'A' + 27,
        _ => unreachable!("wrong char"),
    }
}

impl Solution<Day3> for Aoc2022 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Vec<Vec<u8>>) -> u32 {
        let mut res = 0;
        for line in input {
            assert_eq!(line.len() % 2, 0);

            let (left, right) = line.split_at(line.len() / 2);
            let left: HashSet<_> = left.iter().copied().collect();
            let right: HashSet<_> = right.iter().copied().collect();
            res += left
                .intersection(&right)
                .map(|&c| priority(c) as u32)
                .sum::<u32>();
        }
        res
    }

    fn part2(input: &Vec<Vec<u8>>) -> u32 {
        let mut res = 0;
        for chunks in input.iter().chunks(3).into_iter() {
            let common = chunks
                .map(|line| line.iter().copied().collect::<HashSet<_>>())
                .reduce(|accum, item| accum.intersection(&item).copied().collect())
                .unwrap_or_default();
            assert_eq!(common.len(), 1);

            let common = common.into_iter().next().unwrap();
            res += priority(common) as u32;
        }
        res
    }
}
