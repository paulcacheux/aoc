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

const RUCKSACK_WIDTH: usize = 26 * 2;

struct Rucksack {
    priorities: [bool; RUCKSACK_WIDTH],
}

impl Rucksack {
    fn new(s: &[u8]) -> Self {
        let mut r = Rucksack {
            priorities: [false; RUCKSACK_WIDTH],
        };

        for &c in s {
            let p = priority(c);
            r.priorities[p as usize - 1] = true;
        }

        r
    }

    fn intersect(&mut self, other: &Self) {
        for (p, o) in self.priorities.iter_mut().zip(other.priorities) {
            *p = *p && o;
        }
    }

    fn first_priority(&self) -> u8 {
        self.priorities.iter().find_position(|&&b| b).unwrap().0 as u8 + 1
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
        input
            .iter()
            .map(|line| {
                let (left, right) = line.split_at(line.len() / 2);
                let mut left = Rucksack::new(left);
                let right = Rucksack::new(right);
                left.intersect(&right);
                left.first_priority() as u32
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
                    .map(|chunk| Rucksack::new(chunk))
                    .reduce(|mut accum, item| {
                        accum.intersect(&item);
                        accum
                    })
                    .unwrap();
                common.first_priority() as u32
            })
            .sum()
    }
}
