use std::collections::HashMap;

use crate::aoc2023::Aoc2023;
use crate::traits::days::Day12;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spring {
    Empty,
    Unknown,
    Damaged,
}

#[derive(Debug)]
pub struct Entry {
    springs: Vec<Spring>,
    counts: Vec<usize>,
}

impl ParseInput<Day12> for Aoc2023 {
    type Parsed = Vec<Entry>;

    fn parse_input(input: &str) -> Self::Parsed {
        input
            .lines()
            .map(|line| {
                let (springs, counts) = line.split_once(' ').unwrap();
                let springs = springs
                    .bytes()
                    .map(|b| match b {
                        b'.' => Spring::Empty,
                        b'?' => Spring::Unknown,
                        b'#' => Spring::Damaged,
                        _ => unreachable!(),
                    })
                    .collect();
                let counts = counts.split(',').map(|c| c.parse().unwrap()).collect();
                Entry { springs, counts }
            })
            .collect()
    }
}

impl Solution<Day12> for Aoc2023 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Vec<Entry>) -> usize {
        let mut counter = 0;
        for entry in input {
            let mut solver = Solver::new(entry);
            counter += solver.solve();
        }
        counter
    }

    fn part2(input: &Vec<Entry>) -> usize {
        let extended_input: Vec<_> = input
            .iter()
            .map(|entry| {
                let mut springs = entry.springs.clone();
                let mut counts = entry.counts.clone();

                for _ in 0..4 {
                    springs.push(Spring::Unknown);
                    springs.extend_from_slice(&entry.springs);
                    counts.extend_from_slice(&entry.counts);
                }

                Entry { springs, counts }
            })
            .collect();

        let mut counter = 0;
        for entry in &extended_input {
            let mut solver = Solver::new(entry);
            counter += solver.solve();
        }
        counter
    }
}

struct Solver<'e> {
    entry: &'e Entry,
    cache: HashMap<(usize, usize), usize>,
}

impl<'e> Solver<'e> {
    fn new(entry: &'e Entry) -> Self {
        Solver {
            entry,
            cache: Default::default(),
        }
    }

    fn move_ahead(&mut self, i: usize, j: usize) -> usize {
        // if we go over the counts, we return early
        if j >= self.entry.counts.len() {
            return 0;
        }

        // if we go over the springs, we return early
        if self.entry.springs.len() - i < self.entry.counts[j] {
            return 0;
        }

        if self.entry.springs[i..i + self.entry.counts[j]]
            .iter()
            .any(|&spring| spring == Spring::Empty)
        {
            return 0;
        }

        if self.entry.springs.len() - i == self.entry.counts[j] {
            return self.compute_combinations(self.entry.springs.len(), j + 1);
        }

        if self.entry.springs[i + self.entry.counts[j]] == Spring::Damaged {
            return 0;
        }

        self.compute_combinations(i + self.entry.counts[j] + 1, j + 1)
    }

    fn compute_combinations_inner(&mut self, i: usize, j: usize) -> usize {
        if i >= self.entry.springs.len() {
            if j >= self.entry.counts.len() {
                return 1;
            }
            return 0;
        }

        match self.entry.springs[i] {
            Spring::Empty => self.compute_combinations(i + 1, j),
            Spring::Damaged => self.move_ahead(i, j),
            Spring::Unknown => self.compute_combinations(i + 1, j) + self.move_ahead(i, j),
        }
    }

    fn compute_combinations(&mut self, i: usize, j: usize) -> usize {
        if let Some(res) = self.cache.get(&(i, j)) {
            return *res;
        }

        let new_res = self.compute_combinations_inner(i, j);
        self.cache.insert((i, j), new_res);
        new_res
    }

    fn solve(&mut self) -> usize {
        self.compute_combinations(0, 0)
    }
}
