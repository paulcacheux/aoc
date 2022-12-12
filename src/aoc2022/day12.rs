use ahash::HashMap;

use crate::aoc2022::grid::Grid;
use crate::aoc2022::Aoc2022;
use crate::traits::days::Day12;
use crate::traits::ParseInput;
use crate::traits::Solution;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct Input {
    start: (usize, usize),
    end: (usize, usize),
    grid: Grid<u8>,
}

impl ParseInput<Day12> for Aoc2022 {
    type Parsed = Input;

    fn parse_input(input: &str) -> Self::Parsed {
        let grid = Grid::parse(input, |c| c as u8);
        let mut start = (0, 0);
        let mut end = (0, 0);
        for (x, y, val) in grid.iter() {
            match val {
                b'S' => start = (x, y),
                b'E' => end = (x, y),
                _ => {}
            }
        }
        Input { start, end, grid }
    }
}

fn start_end_mapping(c: &u8) -> u8 {
    match *c {
        b'S' => b'a',
        b'E' => b'z',
        other => other,
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[inline]
fn dijkstra<F, N>(
    grid: &Grid<u8>,
    start: (usize, usize),
    end_check: F,
    neighbor_validate: N,
) -> Option<u32>
where
    F: Fn((usize, usize)) -> bool,
    N: Fn((usize, usize), (usize, usize)) -> bool,
{
    let mut dist = HashMap::default();
    let mut open_queue = BinaryHeap::new();

    dist.insert(start, 0);
    open_queue.push(State {
        cost: 0,
        position: start,
    });

    while let Some(current) = open_queue.pop() {
        if end_check(current.position) {
            return Some(current.cost);
        }

        if dist
            .get(&current.position)
            .map(|&c| current.cost > c)
            .unwrap_or(false)
        {
            continue;
        }

        for next_pos in grid.get_neighbors(current.position.0, current.position.1) {
            if neighbor_validate(current.position, next_pos) {
                let next = State {
                    cost: current.cost + 1,
                    position: next_pos,
                };

                if dist
                    .get(&next_pos)
                    .map(|&dnp| next.cost < dnp)
                    .unwrap_or(true)
                {
                    open_queue.push(next);
                    dist.insert(next_pos, next.cost);
                }
            }
        }
    }
    None
}

impl Solution<Day12> for Aoc2022 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Input) -> u32 {
        dijkstra(
            &input.grid,
            input.start,
            |pos| pos == input.end,
            |current, next_pos| {
                start_end_mapping(input.grid.get(next_pos.0, next_pos.1))
                    <= start_end_mapping(input.grid.get(current.0, current.1)) + 1
            },
        )
        .unwrap()
    }

    fn part2(input: &Input) -> u32 {
        dijkstra(
            &input.grid,
            input.end,
            |pos| start_end_mapping(input.grid.get(pos.0, pos.1)) == b'a',
            |current, next_pos| {
                start_end_mapping(input.grid.get(next_pos.0, next_pos.1)) + 1
                    >= start_end_mapping(input.grid.get(current.0, current.1))
            },
        )
        .unwrap()
    }
}
