use std::collections::HashSet;

use crate::aoc2021::Aoc2021;
use advent_of_code_traits::days::Day13;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;
use std::fmt;

#[derive(Debug)]
pub enum Fold {
    X(u32),
    Y(u32),
}

#[derive(Debug)]
pub struct PuzzleInput {
    dots: Vec<(u32, u32)>,
    folds: Vec<Fold>,
}

const FOLD_X_START: &str = "fold along x=";
const FOLD_Y_START: &str = "fold along y=";

impl ParseInput<Day13> for Aoc2021 {
    type Parsed = PuzzleInput;

    fn parse_input(input: &str) -> PuzzleInput {
        let mut dots = Vec::new();
        let mut folds = Vec::new();
        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if let Some(s) = line.strip_prefix(FOLD_X_START) {
                let axis = s.parse().unwrap();
                folds.push(Fold::X(axis));
            } else if let Some(s) = line.strip_prefix(FOLD_Y_START) {
                let axis = s.parse().unwrap();
                folds.push(Fold::Y(axis));
            } else {
                let mut parts = line.split(',');
                let x = parts.next().unwrap().parse().unwrap();
                let y = parts.next().unwrap().parse().unwrap();
                dots.push((x, y));
            }
        }
        PuzzleInput { dots, folds }
    }
}

struct State {
    dots: Vec<(u32, u32)>,
}

impl State {
    fn fold_y(&mut self, fy: u32) {
        let fy = fy as i64;
        for (_, y) in &mut self.dots {
            let delta = (*y as i64) - fy;
            if delta > 0 {
                let new_y = fy - delta;
                *y = new_y as u32;
            }
        }
    }

    fn fold_x(&mut self, fx: u32) {
        let fx = fx as i64;
        for (x, _) in &mut self.dots {
            let delta = (*x as i64) - fx;
            if delta > 0 {
                let new_x = fx - delta;
                *x = new_x as u32;
            }
        }
    }

    fn count(&self) -> usize {
        let counter: HashSet<_> = self.dots.iter().copied().collect();
        counter.len()
    }

    fn into_display(self) -> StateDisplay {
        let width = (self.dots.iter().map(|d| d.0).max().unwrap() + 1) as usize;
        let height = (self.dots.iter().map(|d| d.1).max().unwrap() + 1) as usize;

        let mut display_dots = vec![false; width * height];

        for (x, y) in self.dots {
            let index = y as usize * width + x as usize;
            display_dots[index] = true;
        }

        StateDisplay {
            dots: display_dots,
            width,
            height,
        }
    }
}

pub struct StateDisplay {
    dots: Vec<bool>,
    width: usize,
    height: usize,
}

impl fmt::Display for StateDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let value = self.dots[y * self.width + x];
                write!(f, "{}", if value { '#' } else { ' ' })?;
            }
            if y != self.height - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Solution<Day13> for Aoc2021 {
    type Part1Output = usize;
    type Part2Output = StateDisplay;

    fn part1(input: &PuzzleInput) -> usize {
        let mut state = State {
            dots: input.dots.clone(),
        };

        match input.folds[0] {
            Fold::X(fx) => {
                state.fold_x(fx);
            }
            Fold::Y(fy) => {
                state.fold_y(fy);
            }
        }

        state.count()
    }

    fn part2(input: &PuzzleInput) -> StateDisplay {
        let mut state = State {
            dots: input.dots.clone(),
        };

        for fold in &input.folds {
            match fold {
                Fold::X(fx) => {
                    state.fold_x(*fx);
                }
                Fold::Y(fy) => {
                    state.fold_y(*fy);
                }
            }
        }
        state.into_display()
    }
}
