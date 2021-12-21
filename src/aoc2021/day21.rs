use ahash::AHashMap;

use crate::aoc2021::Aoc2021;
use advent_of_code_traits::days::Day21;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;
use regex::Regex;

#[derive(Debug)]
pub struct PuzzleInput {
    player1: u8,
    player2: u8,
}

impl ParseInput<Day21> for Aoc2021 {
    type Parsed = PuzzleInput;

    fn parse_input(input: &str) -> PuzzleInput {
        let re = Regex::new(r"Player (\d+) starting position: (\d+)").unwrap();
        let mut puzzle_input = PuzzleInput {
            player1: 0,
            player2: 0,
        };

        for line in input.lines() {
            let line = line.trim();
            if let Some(captures) = re.captures(line) {
                let player: usize = captures[1].parse().unwrap();
                let position = captures[2].parse().unwrap();

                match player {
                    1 => puzzle_input.player1 = position,
                    2 => puzzle_input.player2 = position,
                    _ => unreachable!(),
                }
            }
        }
        puzzle_input
    }
}

#[derive(Debug)]
struct Part1State {
    players_position: Vec<u8>,
    players_scores: Vec<u32>,
    turn: usize,

    dice_state: u32,
    dice_counter: usize,
}

fn add_strange_mod10(lhs: u8, rhs: u32) -> u8 {
    ((lhs as u32 + rhs - 1) % 10 + 1) as u8
}

impl Part1State {
    fn new(input: &PuzzleInput) -> Self {
        Part1State {
            players_position: vec![input.player1, input.player2],
            players_scores: vec![0, 0],
            turn: 0,
            dice_state: 1,
            dice_counter: 0,
        }
    }

    fn next_dice(&mut self) -> u32 {
        let value = self.dice_state;
        self.dice_state += 1;
        if self.dice_state > 100 {
            self.dice_state = 1;
        }
        self.dice_counter += 1;
        value
    }

    fn step(&mut self) {
        let (a, b, c) = (self.next_dice(), self.next_dice(), self.next_dice());
        let value = a + b + c;

        let new_position = add_strange_mod10(self.players_position[self.turn], value);
        self.players_scores[self.turn] += new_position as u32;
        self.players_position[self.turn] = new_position;
        self.turn = (self.turn + 1) % 2;
    }

    fn is_over(&mut self) -> bool {
        for score in &self.players_scores {
            if *score >= 1000 {
                return true;
            }
        }
        false
    }

    fn score(&self) -> usize {
        let min = self.players_scores.iter().copied().min().unwrap_or(0);
        min as usize * self.dice_counter
    }
}

#[derive(Default)]
struct Part2StateCache {
    cache: AHashMap<Part2State, (u64, u64)>,
}

impl Part2StateCache {
    fn get_count(&mut self, state: &Part2State) -> (u64, u64) {
        if let Some(count) = self.cache.get(state) {
            return *count;
        }

        let res = if state.scores[0] >= 21 {
            (1, 0)
        } else if state.scores[1] >= 21 {
            (0, 1)
        } else {
            let mut sum = (0, 0);
            for a in 1..=3 {
                for b in 1..=3 {
                    for c in 1..=3 {
                        let value = a + b + c;
                        let (p1, p2) = self.get_count(&state.next_state(value));
                        sum = (sum.0 + p1, sum.1 + p2);
                    }
                }
            }
            sum
        };

        self.cache.insert(*state, res);
        res
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Part2State {
    positions: [u8; 2],
    scores: [u32; 2],
    turn: u8,
}

impl Part2State {
    fn next_state(&self, dice_sum: u32) -> Self {
        let turn = self.turn as usize;
        let mut new_state = *self;
        let new_position = add_strange_mod10(self.positions[turn], dice_sum);
        new_state.scores[turn] += new_position as u32;
        new_state.positions[turn] = new_position;
        new_state.turn = (self.turn + 1) % 2;
        new_state
    }
}

impl Solution<Day21> for Aoc2021 {
    type Part1Output = usize;
    type Part2Output = u64;

    fn part1(input: &PuzzleInput) -> usize {
        let mut state = Part1State::new(input);
        while !state.is_over() {
            state.step();
        }
        state.score()
    }

    fn part2(input: &PuzzleInput) -> u64 {
        let mut state_cache = Part2StateCache::default();
        let start_state = Part2State {
            positions: [input.player1, input.player2],
            scores: [0, 0],
            turn: 0,
        };

        let (p1, p2) = state_cache.get_count(&start_state);
        if p1 >= p2 {
            p1
        } else {
            p2
        }
    }
}
