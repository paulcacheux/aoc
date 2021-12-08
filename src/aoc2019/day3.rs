use std::collections::{HashMap, HashSet};

use crate::aoc2019::Aoc2019;
use advent_of_code_traits::days::Day3;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn to_delta(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Debug)]
pub struct Order {
    direction: Direction,
    count: u32,
}

#[derive(Debug)]
pub struct PuzzleInput {
    line1: Vec<Order>,
    line2: Vec<Order>,
}

impl ParseInput<Day3> for Aoc2019 {
    type Parsed = PuzzleInput;

    fn parse_input(input: &str) -> PuzzleInput {
        fn parse_orders(line: &str) -> Vec<Order> {
            line.split(',')
                .map(|s| {
                    let s = s.trim();
                    let direction = match s.as_bytes()[0] {
                        b'U' => Direction::Up,
                        b'D' => Direction::Down,
                        b'L' => Direction::Left,
                        b'R' => Direction::Right,
                        _ => unreachable!(),
                    };
                    let count = s[1..].parse().unwrap();
                    Order { direction, count }
                })
                .collect()
        }

        let mut lines_iter = input.lines();
        let line1 = lines_iter.next().unwrap();
        let line2 = lines_iter.next().unwrap();

        PuzzleInput {
            line1: parse_orders(line1),
            line2: parse_orders(line2),
        }
    }
}

fn visit_line<F: FnMut(i32, i32, usize)>(orders: &[Order], mut visitor: F) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut step = 0;

    for order in orders {
        let count = order.count as i32;
        let (dx, dy) = order.direction.to_delta();

        for _ in 0..count {
            step += 1;
            x += dx;
            y += dy;

            visitor(x, y, step);
        }
    }
}

impl Solution<Day3> for Aoc2019 {
    type Part1Output = u32;
    type Part2Output = usize;

    fn part1(input: &PuzzleInput) -> u32 {
        let mut visited = HashSet::new();
        visit_line(&input.line1, |x, y, _| {
            visited.insert((x, y));
        });

        let mut min_distance = None;

        visit_line(&input.line2, |x, y, _| {
            if visited.contains(&(x, y)) {
                let dist = (x.abs() + y.abs()) as u32;
                if let Some(md) = min_distance {
                    if dist < md {
                        min_distance = Some(dist);
                    }
                } else {
                    min_distance = Some(dist);
                }
            }
        });

        min_distance.unwrap()
    }

    fn part2(input: &PuzzleInput) -> usize {
        let mut visited = HashMap::new();
        visit_line(&input.line1, |x, y, step| {
            visited.insert((x, y), step);
        });

        let mut min_distance = None;

        visit_line(&input.line2, |x, y, line2_step| {
            if let Some(line1_step) = visited.get(&(x, y)) {
                let dist = line1_step + line2_step;
                if let Some(md) = min_distance {
                    if dist < md {
                        min_distance = Some(dist);
                    }
                } else {
                    min_distance = Some(dist);
                }
            }
        });

        min_distance.unwrap()
    }
}
