use itertools::Itertools;

use crate::aoc2023::Aoc2023;
use crate::traits::days::Day6;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug)]
pub struct Entry {
    time: u64,
    distance: u64,
}

impl Entry {
    fn count_ways(&self) -> u64 {
        let mut count = 0;
        for prep in 0..=self.time {
            let distance = (self.time - prep) * prep;
            if distance > self.distance {
                count += 1;
            }
        }
        count
    }
}

impl ParseInput<Day6> for Aoc2023 {
    type Parsed = Vec<Entry>;

    fn parse_input(input: &str) -> Self::Parsed {
        let mut iter = input.lines();

        let times = iter.next().unwrap().strip_prefix("Time:").unwrap();
        let distances = iter.next().unwrap().strip_prefix("Distance:").unwrap();

        times
            .split_ascii_whitespace()
            .map(|part| part.parse::<u64>().unwrap())
            .zip(
                distances
                    .split_ascii_whitespace()
                    .map(|part| part.parse::<u64>().unwrap()),
            )
            .map(|(time, distance)| Entry { time, distance })
            .collect()
    }
}

impl Solution<Day6> for Aoc2023 {
    type Part1Output = u64;
    type Part2Output = u64;

    fn part1(input: &Vec<Entry>) -> u64 {
        let mut score = 1;
        for entry in input {
            score *= entry.count_ways();
        }
        score
    }

    fn part2(input: &Vec<Entry>) -> u64 {
        let time: u64 = input
            .iter()
            .map(|entry| entry.time.to_string())
            .join("")
            .parse()
            .unwrap();
        let distance: u64 = input
            .iter()
            .map(|entry| entry.distance.to_string())
            .join("")
            .parse()
            .unwrap();

        Entry { time, distance }.count_ways()
    }
}
