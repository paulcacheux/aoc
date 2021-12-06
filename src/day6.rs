use crate::aoc::Aoc2021;
use advent_of_code_traits::days::Day6;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;
use std::collections::HashMap;

impl ParseInput<Day6> for Aoc2021 {
    type Parsed = Vec<u8>;

    fn parse_input(input: &str) -> Vec<u8> {
        input
            .trim()
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect()
    }
}

struct State {
    lives: Vec<(u8, usize)>,
}

impl State {
    fn new(input: &[u8]) -> Self {
        let mut counter: HashMap<u8, usize> = HashMap::new();
        for v in input {
            *counter.entry(*v).or_default() += 1;
        }

        let lives = counter.into_iter().collect();
        Self { lives }
    }

    fn next_state(&mut self) {
        let mut new_count: usize = 0;
        for (v, count) in self.lives.iter_mut() {
            if *v == 0 {
                *v = 6;
                new_count += *count;
            } else {
                *v -= 1;
            }
        }
        self.lives.push((8, new_count));
    }

    fn next_n_state(&mut self, n: usize) {
        for _ in 0..n {
            self.next_state();
        }
    }

    fn count(&self) -> usize {
        self.lives.iter().map(|(_, count)| *count).sum()
    }
}

impl Solution<Day6> for Aoc2021 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Vec<u8>) -> usize {
        let mut state = State::new(input);
        state.next_n_state(80);
        state.count()
    }

    fn part2(input: &Vec<u8>) -> usize {
        let mut state = State::new(input);
        state.next_n_state(256);
        state.count()
    }
}
