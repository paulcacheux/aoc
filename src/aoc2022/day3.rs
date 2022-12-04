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

#[derive(Clone, Debug, Copy)]
struct Rucksack {
    priorities: u64,
}

impl Rucksack {
    fn new(s: &[u8]) -> Self {
        let mut r = Rucksack { priorities: 0 };

        for &c in s {
            let p = priority(c);
            r.set(p as usize - 1)
        }

        r
    }

    #[inline]
    fn set(&mut self, index: usize) {
        self.priorities |= 0b1 << index;
    }

    #[inline]
    fn get(&self, index: usize) -> bool {
        self.priorities & (0b1 << index) != 0
    }

    fn intersect(left: Self, right: Self) -> Self {
        Rucksack {
            priorities: left.priorities & right.priorities,
        }
    }

    fn first_priority(&self) -> u8 {
        for i in 0..RUCKSACK_WIDTH {
            if self.get(i) {
                return i as u8 + 1;
            }
        }
        unreachable!("at least one priority")
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
                let left = Rucksack::new(left);
                let right = Rucksack::new(right);
                Rucksack::intersect(left, right).first_priority() as u32
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
                    .reduce(Rucksack::intersect)
                    .unwrap();
                common.first_priority() as u32
            })
            .sum()
    }
}
