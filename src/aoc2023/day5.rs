use crate::aoc2023::Aoc2023;
use crate::traits::days::Day5;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug)]
struct MappingRange {
    destination: u64,
    source: u64,
    len: u64,
}

impl MappingRange {
    fn transform(&self, value: u64) -> Option<u64> {
        if self.source <= value && value < self.source + self.len {
            Some(self.destination + (value - self.source))
        } else {
            None
        }
    }
}

#[derive(Debug, Default)]
pub struct Mapping {
    from: String,
    to: String,
    ranges: Vec<MappingRange>,
}

impl Mapping {
    fn transform(&self, value: u64) -> u64 {
        for range in &self.ranges {
            if let Some(res) = range.transform(value) {
                return res;
            }
        }
        value
    }
}

#[derive(Debug)]
pub struct Input {
    seeds: Vec<u64>,
    mappings: Vec<Mapping>,
}

impl ParseInput<Day5> for Aoc2023 {
    type Parsed = Input;

    fn parse_input(input: &str) -> Self::Parsed {
        let mut lines_iter = input.lines();

        let seeds = lines_iter
            .next()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split_ascii_whitespace()
            .map(|value| value.parse().unwrap())
            .collect();

        lines_iter.next().unwrap(); // skip empty line

        let mut mappings = Vec::new();
        let mut current = Mapping::default();
        for line in lines_iter {
            if let Some(desc) = line.strip_suffix(" map:") {
                let (from, to) = desc.split_once("-to-").unwrap();
                current.from = from.to_owned();
                current.to = to.to_owned();
            } else if line.trim().is_empty() {
                mappings.push(current);
                current = Mapping::default();
            } else {
                let mut values = line.split_ascii_whitespace();
                let destination = values.next().unwrap().parse().unwrap();
                let source = values.next().unwrap().parse().unwrap();
                let len = values.next().unwrap().parse().unwrap();
                current.ranges.push(MappingRange {
                    destination,
                    source,
                    len,
                })
            }
        }

        if !current.from.is_empty() {
            mappings.push(current);
        }

        Input { seeds, mappings }
    }
}

impl Solution<Day5> for Aoc2023 {
    type Part1Output = u64;
    type Part2Output = u64;

    fn part1(input: &Input) -> u64 {
        input
            .seeds
            .iter()
            .map(|&seed| {
                let mut current = seed;
                for mapping in &input.mappings {
                    current = mapping.transform(current);
                }
                current
            })
            .min()
            .unwrap()
    }

    fn part2(input: &Input) -> u64 {
        todo!()
    }
}
