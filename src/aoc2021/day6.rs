use crate::aoc2021::Aoc2021;
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
    lives: [usize; 9],
}

impl State {
    fn new(input: &[u8]) -> Self {
        let mut counter: HashMap<u8, usize> = HashMap::new();
        for v in input {
            *counter.entry(*v).or_default() += 1;
        }

        let mut lives = [0; 9];
        for (key, count) in counter {
            assert!(key < 9);
            lives[key as usize] = count;
        }

        Self { lives }
    }

    fn next_state(&mut self) {
        let mut new_lives = [0; 9];
        for (i, live) in new_lives.iter_mut().enumerate() {
            if i == 6 {
                *live = self.lives[i + 1] + self.lives[0];
            } else if i == 8 {
                *live = self.lives[0];
            } else {
                *live = self.lives[i + 1];
            }
        }
        self.lives = new_lives;
    }

    fn next_n_state(&mut self, n: usize) {
        for _ in 0..n {
            self.next_state();
        }
    }

    fn count(&self) -> usize {
        self.lives.iter().sum()
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
