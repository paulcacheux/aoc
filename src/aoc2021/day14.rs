use std::collections::HashMap;

use crate::aoc2021::Aoc2021;
use advent_of_code_traits::days::Day14;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;
use itertools::Itertools;

#[derive(Debug)]
pub struct PuzzleInput {
    template: String,
    pairs: Vec<((char, char), char)>,
}

impl ParseInput<Day14> for Aoc2021 {
    type Parsed = PuzzleInput;

    fn parse_input(input: &str) -> PuzzleInput {
        let mut lines = input.lines();

        let template = lines.next().unwrap().to_owned();
        lines.next(); // skip empty

        let pairs = lines
            .map(|line| {
                let mut parts = line.split(" -> ");
                let lhs = parts.next().unwrap().as_bytes();
                let rhs = parts.next().unwrap().as_bytes();

                assert_eq!(lhs.len(), 2);
                assert_eq!(rhs.len(), 1);

                ((lhs[0] as char, lhs[1] as char), rhs[0] as char)
            })
            .collect();

        PuzzleInput { template, pairs }
    }
}

#[derive(Debug)]
struct State {
    pairs: HashMap<(char, char), usize>,
}

impl State {
    fn step(self, rules: &HashMap<(char, char), char>) -> State {
        let mut new_pairs = HashMap::new();
        for (pair @ (a, b), count) in self.pairs {
            let res = rules[&pair];
            *new_pairs.entry((a, res)).or_default() += count;
            *new_pairs.entry((res, b)).or_default() += count;
        }
        State { pairs: new_pairs }
    }

    fn counts(&self) -> HashMap<char, usize> {
        let mut counter: HashMap<_, (usize, usize)> = HashMap::new();
        for (&(a, b), &count) in &self.pairs {
            counter.entry(a).or_default().0 += count;
            counter.entry(b).or_default().1 += count;
        }

        counter
            .into_iter()
            .map(|(c, (left, right))| (c, std::cmp::max(left, right)))
            .collect()
    }
}

fn run(input: &PuzzleInput, steps: usize) -> usize {
    let rules: HashMap<_, _> = input.pairs.iter().copied().collect();

    let mut state_pairs = HashMap::new();
    for (a, b) in input.template.chars().tuple_windows() {
        *state_pairs.entry((a, b)).or_default() += 1;
    }
    let mut state = State { pairs: state_pairs };

    for _ in 0..steps {
        state = state.step(&rules);
    }

    match state.counts().values().minmax() {
        itertools::MinMaxResult::NoElements | itertools::MinMaxResult::OneElement(_) => {
            unreachable!()
        }
        itertools::MinMaxResult::MinMax(min, max) => max - min,
    }
}

impl Solution<Day14> for Aoc2021 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &PuzzleInput) -> usize {
        run(input, 10)
    }

    fn part2(input: &PuzzleInput) -> usize {
        run(input, 40)
    }
}
