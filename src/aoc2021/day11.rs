use crate::aoc2021::Aoc2021;
use crate::traits::days::Day11;
use crate::traits::ParseInput;
use crate::traits::Solution;
use itertools::iproduct;
use std::fmt;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

#[derive(Debug, Clone)]
pub struct PuzzleInput {
    values: Vec<u8>,
}

impl PuzzleInput {
    fn get(&self, x: usize, y: usize) -> u8 {
        self.values[y * WIDTH + x]
    }

    fn set(&mut self, x: usize, y: usize, value: u8) {
        self.values[y * WIDTH + x] = value;
    }

    fn inc(&mut self, x: usize, y: usize) {
        self.values[y * WIDTH + x] += 1;
    }
}

impl fmt::Display for PuzzleInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                write!(f, "{}", self.get(x, y))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl ParseInput<Day11> for Aoc2021 {
    type Parsed = PuzzleInput;

    fn parse_input(input: &str) -> PuzzleInput {
        let values: Vec<_> = input
            .lines()
            .flat_map(|line| line.trim().chars().map(|c| c.to_digit(10).unwrap() as u8))
            .collect();
        assert_eq!(values.len(), WIDTH * HEIGHT);
        PuzzleInput { values }
    }
}

fn get_neighbors(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    let x = x as i32;
    let y = y as i32;
    iproduct!([-1i32, 0, 1], [-1i32, 0, 1])
        .filter(|&(dx, dy)| dx != 0 || dy != 0)
        .filter_map(move |(dx, dy)| {
            let rx = x + dx;
            let ry = y + dy;

            if 0 <= rx && rx < WIDTH as _ && 0 <= ry && ry < HEIGHT as _ {
                Some((rx as usize, ry as usize))
            } else {
                None
            }
        })
}

fn next_step(state: &mut PuzzleInput) -> usize {
    for v in &mut state.values {
        *v += 1;
    }

    let mut flash_counter = 0;
    let mut should_continue = true;
    while should_continue {
        should_continue = false;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if state.get(x, y) > 9 {
                    flash_counter += 1;
                    for (nx, ny) in get_neighbors(x, y) {
                        if state.get(nx, ny) != 0 {
                            state.inc(nx, ny);
                        }
                    }
                    state.set(x, y, 0);
                    should_continue = true;
                }
            }
        }
    }
    flash_counter
}

impl Solution<Day11> for Aoc2021 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &PuzzleInput) -> usize {
        let mut state = input.clone();
        let mut flash_counter = 0;
        for _ in 0..100 {
            flash_counter += next_step(&mut state);
        }
        flash_counter
    }

    fn part2(input: &PuzzleInput) -> usize {
        let mut state = input.clone();
        let mut step = 0;
        loop {
            step += 1;
            let flashes = next_step(&mut state);
            if flashes == WIDTH * HEIGHT {
                return step;
            }
        }
    }
}
