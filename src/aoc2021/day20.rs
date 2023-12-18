use std::collections::HashMap;
use std::fmt;
use std::ops::RangeInclusive;

use crate::aoc2021::Aoc2021;
use crate::traits::days::Day20;
use crate::traits::ParseInput;
use crate::traits::Solution;
use itertools::Itertools;
use itertools::MinMaxResult;

#[derive(Debug)]
pub struct PuzzleInput {
    replacement_rules: Vec<bool>,
    grid: HashMap<(i32, i32), bool>,
}

impl ParseInput<Day20> for Aoc2021 {
    type Parsed = PuzzleInput;

    fn parse_input(input: &str) -> PuzzleInput {
        let mut replacement_line = String::new();
        let mut in_replacement = true;
        let mut grid = HashMap::new();

        let mut line_counter = 0;
        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                in_replacement = false;
                line_counter = 0;
                continue;
            }

            if in_replacement {
                replacement_line.push_str(line);
            } else {
                for (x, c) in line.chars().enumerate() {
                    let value = match c {
                        '.' => false,
                        '#' => true,
                        _ => unreachable!(),
                    };
                    grid.insert((x as i32, line_counter), value);
                }

                line_counter += 1;
            }
        }

        let replacement_rules = replacement_line
            .chars()
            .map(|c| match c {
                '.' => false,
                '#' => true,
                _ => unreachable!(),
            })
            .collect();

        PuzzleInput {
            replacement_rules,
            grid,
        }
    }
}

#[derive(Debug)]
struct Grid {
    inner: HashMap<(i32, i32), bool>,
    background: bool,
}

impl Grid {
    fn new(inner: HashMap<(i32, i32), bool>) -> Self {
        Grid {
            inner,
            background: false,
        }
    }

    fn get(&self, x: i32, y: i32) -> bool {
        self.inner.get(&(x, y)).copied().unwrap_or(self.background)
    }

    fn bounds(&self) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
        fn build_range(res: MinMaxResult<i32>) -> RangeInclusive<i32> {
            match res {
                MinMaxResult::NoElements => unreachable!(),
                MinMaxResult::OneElement(elem) => (elem - 1)..=(elem + 1),
                MinMaxResult::MinMax(s, e) => (s - 1)..=(e + 1),
            }
        }

        let xres = self.inner.keys().map(|(x, _)| *x).minmax();
        let xrange = build_range(xres);
        let yres = self.inner.keys().map(|(_, y)| *y).minmax();
        let yrange = build_range(yres);

        (xrange, yrange)
    }

    fn next_grid(&self, rules: &[bool]) -> Grid {
        let (xbounds, ybounds) = self.bounds();
        let mut new_inner = HashMap::new();

        for y in ybounds {
            for x in xbounds.clone() {
                let mut bits = 0;
                for dy in [-1, 0, 1] {
                    for dx in [-1, 0, 1] {
                        let bit = self.get(x + dx, y + dy) as usize;
                        bits = (bits << 1) | bit;
                    }
                }
                new_inner.insert((x, y), rules[bits]);
            }
        }

        let new_background = if self.background {
            rules[511]
        } else {
            rules[0]
        };

        Grid {
            inner: new_inner,
            background: new_background,
        }
    }

    fn count_lit(&self) -> Option<usize> {
        if self.background {
            None
        } else {
            Some(self.inner.values().filter(|b| **b).count())
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (xbounds, ybounds) = self.bounds();
        for y in ybounds {
            for x in xbounds.clone() {
                if self.get(x, y) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Solution<Day20> for Aoc2021 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &PuzzleInput) -> usize {
        let mut grid = Grid::new(input.grid.clone());
        for _ in 0..2 {
            grid = grid.next_grid(&input.replacement_rules);
        }
        grid.count_lit().unwrap()
    }

    fn part2(input: &PuzzleInput) -> usize {
        let mut grid = Grid::new(input.grid.clone());
        for _ in 0..50 {
            grid = grid.next_grid(&input.replacement_rules);
        }
        grid.count_lit().unwrap()
    }
}
