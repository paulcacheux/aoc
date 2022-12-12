use crate::aoc2022::grid::Grid;
use crate::aoc2022::Aoc2022;
use crate::traits::days::Day12;
use crate::traits::ParseInput;
use crate::traits::Solution;
use std::cmp::Ordering;

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
fn backtrack_parents(
    parents: Grid<Option<(usize, usize)>>,
    mut current: (usize, usize),
    start: (usize, usize),
) -> Option<u32> {
    let mut count = 1;
    while let Some(next) = parents.get(current.0, current.1) {
        if *next == start {
            return Some(count);
        }
        current = *next;
        count += 1;
    }
    None
}

#[inline]
fn bfs<F, N>(
    grid: &Grid<u8>,
    start: (usize, usize),
    end_check: F,
    neighbor_validate: N,
) -> Option<u32>
where
    F: Fn((usize, usize)) -> bool,
    N: Fn((usize, usize), (usize, usize)) -> bool,
{
    let mut parents = Grid::new(grid.width, grid.height, None);
    let mut open_queue = Queue::with_capacity(grid.width * grid.height);
    open_queue.push_back(start);

    while let Some(current) = open_queue.pop_front() {
        if end_check(current) {
            return backtrack_parents(parents, current, start);
        }

        for next_pos in grid.get_neighbors(current.0, current.1) {
            if neighbor_validate(current, next_pos) && parents.get(next_pos.0, next_pos.1).is_none()
            {
                open_queue.push_back(next_pos);
                parents.set(next_pos.0, next_pos.1, Some(current));
            }
        }
    }
    None
}

#[derive(Default)]
struct Queue<T: Copy> {
    inner: Vec<T>,
    index: usize,
}

impl<T: Copy> Queue<T> {
    fn with_capacity(cap: usize) -> Self {
        Queue {
            inner: Vec::with_capacity(cap),
            index: 0,
        }
    }

    fn push_back(&mut self, value: T) {
        self.inner.push(value);
    }

    fn pop_front(&mut self) -> Option<T> {
        if self.index < self.inner.len() {
            let value = self.inner[self.index];
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}

impl Solution<Day12> for Aoc2022 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Input) -> u32 {
        bfs(
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
        bfs(
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
