use crate::aoc2022::Aoc2022;
use advent_of_code_traits::days::Day3;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;
use itertools::Itertools;

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

fn intersection_p1(left: &[u8], right: &[u8]) -> Option<u8> {
    for &y in right {
        for &x in left {
            if x == y {
                return Some(x);
            }
        }
    }
    None
}

fn intersection_p2(left: &[u8], right: &[u8]) -> Vec<u8> {
    let mut res = Vec::with_capacity(left.len());
    for &y in right {
        for &x in left {
            if x == y {
                res.push(x);
            }
        }
    }
    res.sort();
    res.dedup();
    res
}

impl Solution<Day3> for Aoc2022 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Vec<Vec<u8>>) -> u32 {
        input
            .iter()
            .map(|line| {
                let (left, right) = line.split_at(line.len() / 2);
                priority(intersection_p1(left, right).unwrap_or_default()) as u32
            })
            .sum()
    }

    fn part2(input: &Vec<Vec<u8>>) -> u32 {
        input
            .iter()
            .chunks(3)
            .into_iter()
            .map(|chunks| {
                let common = chunks
                    .cloned()
                    .reduce(|accum, item| intersection_p2(&accum, &item))
                    .unwrap_or_default();
                priority(common[0]) as u32
            })
            .sum()
    }
}
