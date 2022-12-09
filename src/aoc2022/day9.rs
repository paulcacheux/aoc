use crate::aoc2022::Aoc2022;
use crate::traits::days::Day9;
use crate::traits::ParseInput;
use crate::traits::Solution;

use ahash::HashSet;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn offset(self) -> (i32, i32) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
        }
    }
}

#[derive(Debug)]
pub struct Order {
    dir: Direction,
    count: u32,
}

impl ParseInput<Day9> for Aoc2022 {
    type Parsed = Vec<Order>;

    fn parse_input(input: &str) -> Self::Parsed {
        let mut orders = Vec::new();
        for line in input.lines() {
            let dirc = line.chars().next().unwrap();
            let count = line[2..].parse().unwrap();
            let dir = match dirc {
                'L' => Direction::Left,
                'R' => Direction::Right,
                'U' => Direction::Up,
                'D' => Direction::Down,
                _ => unreachable!(),
            };
            orders.push(Order { dir, count })
        }
        orders
    }
}

#[derive(Debug)]
struct State<const S: usize> {
    knots: [(i32, i32); S],
}

impl<const S: usize> State<S> {
    fn new() -> Self {
        State { knots: [(0, 0); S] }
    }

    fn sync_tail(&mut self) {
        for i in 0..(S - 1) {
            let head = self.knots[i];
            let tail = self.knots[i + 1];

            let dx = head.0 - tail.0;
            let dy = head.1 - tail.1;

            let (tx, ty) = if dx.abs() == 2 && dy == 0 {
                (dx / 2, dy)
            } else if dx == 0 && dy.abs() == 2 {
                (dx, dy / 2)
            } else if dx.abs() == 2 && dy.abs() == 1 {
                (dx / 2, dy)
            } else if dx.abs() == 1 && dy.abs() == 2 {
                (dx, dy / 2)
            } else if dx.abs() == 2 && dy.abs() == 2 {
                (dx / 2, dy / 2)
            } else {
                (0, 0)
            };

            self.knots[i + 1].0 += tx;
            self.knots[i + 1].1 += ty;
        }
    }
}

impl Solution<Day9> for Aoc2022 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Vec<Order>) -> usize {
        let mut state = State::<2>::new();
        let mut visited = HashSet::default();
        visited.insert((0, 0));

        for order in input {
            let (dx, dy) = order.dir.offset();
            for _ in 0..order.count {
                state.knots[0].0 += dx;
                state.knots[0].1 += dy;

                state.sync_tail();
                visited.insert(state.knots.last().copied().unwrap());
            }
        }
        visited.len()
    }

    fn part2(input: &Vec<Order>) -> usize {
        let mut state = State::<10>::new();
        let mut visited = HashSet::default();
        visited.insert((0, 0));

        for order in input {
            let (dx, dy) = order.dir.offset();
            for _ in 0..order.count {
                state.knots[0].0 += dx;
                state.knots[0].1 += dy;

                state.sync_tail();
                visited.insert(state.knots.last().copied().unwrap());
            }
        }
        visited.len()
    }
}
