use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt;

use crate::aoc2021::Aoc2021;
use advent_of_code_traits::days::Day15;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;

#[derive(Debug)]
pub struct PuzzleInput {
    values: Vec<u8>,
    width: usize,
    height: usize,
}

impl ParseInput<Day15> for Aoc2021 {
    type Parsed = PuzzleInput;

    fn parse_input(input: &str) -> PuzzleInput {
        let mut values = Vec::new();
        let mut width = None;
        let mut height = 0;
        for line in input.lines() {
            height += 1;
            let line = line.trim();
            let line_values: Vec<_> = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect();

            if let Some(w) = width {
                assert_eq!(w, line_values.len());
            } else {
                width = Some(line_values.len());
            }
            values.extend(line_values);
        }

        PuzzleInput {
            values,
            width: width.unwrap(),
            height,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
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

struct Grid {
    values: Vec<u8>,
    local_width: usize,
    local_height: usize,
    multiplier: usize,
}

fn hack_add(base: u8, added: usize) -> u8 {
    let mut current = base;
    for _ in 0..added {
        current += 1;
        if current > 9 {
            current = 1;
        }
    }
    current
}

impl Grid {
    fn new(input: &PuzzleInput, multiplier: usize) -> Self {
        Self {
            values: input.values.clone(),
            local_width: input.width,
            local_height: input.height,
            multiplier,
        }
    }

    fn width(&self) -> usize {
        self.local_width * self.multiplier
    }

    fn height(&self) -> usize {
        self.local_height * self.multiplier
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        assert!(x < self.width());
        assert!(y < self.height());

        let offx = x / self.local_width;
        let offy = y / self.local_height;

        let x = x % self.local_width;
        let y = y % self.local_height;

        hack_add(self.values[y * self.local_width + x], offx + offy)
    }

    fn get_neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let width = self.width();
        let height = self.height();
        assert!(x < width);
        assert!(y < height);
        let x = x as i32;
        let y = y as i32;

        let deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        deltas.into_iter().flat_map(move |(dx, dy)| {
            let rx = x + dx;
            let ry = y + dy;

            if 0 <= rx && rx < width as _ && 0 <= ry && ry < height as _ {
                Some((rx as usize, ry as usize))
            } else {
                None
            }
        })
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                write!(f, "{}", self.get(x, y))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn shortest_path(grid: &Grid) -> Option<usize> {
    let start = (0, 0);
    let goal = (grid.width() - 1, grid.height() - 1);

    let mut dist = HashMap::new();
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            dist.insert((x, y), usize::MAX);
        }
    }
    let mut heap = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(State {
        cost: 0,
        position: start,
    });

    let mut predecessors = HashMap::new();

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }

        if cost > dist[&position] {
            continue;
        }

        let (px, py) = position;
        for neighbor in grid.get_neighbors(px, py) {
            let next = State {
                cost: cost + grid.get(neighbor.0, neighbor.1) as usize,
                position: neighbor,
            };

            if next.cost < dist[&next.position] {
                heap.push(next);
                dist.insert(next.position, next.cost);
                predecessors.insert(next.position, position);
            }
        }
    }
    None
}

impl Solution<Day15> for Aoc2021 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &PuzzleInput) -> usize {
        let grid = Grid::new(input, 1);
        // println!("{}", grid);
        shortest_path(&grid).unwrap()
    }

    fn part2(input: &PuzzleInput) -> usize {
        let grid = Grid::new(input, 5);
        shortest_path(&grid).unwrap()
    }
}
