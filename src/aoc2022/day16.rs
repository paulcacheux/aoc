use crate::aoc2022::Aoc2022;
use crate::traits::days::Day16;
use crate::traits::ParseInput;
use crate::traits::Solution;

use ahash::HashMap;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Valve {
    name: String,
    rate: u32,
    edges: Vec<String>,
}

impl ParseInput<Day16> for Aoc2022 {
    type Parsed = Vec<Valve>;

    fn parse_input(input: &str) -> Self::Parsed {
        let line_re =
            Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ((\w|[, ])+)")
                .unwrap();

        input
            .lines()
            .map(str::trim)
            .map(|line| {
                let captures = line_re.captures(line).unwrap();
                let name = captures.get(1).unwrap().as_str().to_owned();
                let rate = captures.get(2).unwrap().as_str().parse().unwrap();
                let edges = captures
                    .get(3)
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .map(str::to_owned)
                    .collect();
                Valve { name, rate, edges }
            })
            .collect()
    }
}

impl Solution<Day16> for Aoc2022 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Vec<Valve>) -> u32 {}

    fn part2(_input: &Vec<Valve>) -> u32 {
        todo!()
    }
}

#[derive(Debug)]
struct Path {
    nodes: Vec<(String, bool)>,
    score: u32,
}

impl Path {
    fn last(&self) -> &str {
        &self.nodes.last().unwrap().0
    }

    fn time(&self) -> u32 {
        self.nodes.len() as u32
    }
}
