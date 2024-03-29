use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt;

use crate::aoc2021::Aoc2021;
use crate::traits::days::Day15;
use crate::traits::ParseInput;
use crate::traits::Solution;

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

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct State {
    cost: usize,
    position: (usize, usize),
}

struct Grid {
    values: Vec<u8>,
    local_width: usize,
    local_height: usize,
    multiplier: usize,
}

fn add_strange_mod(base: u8, added: usize) -> u8 {
    (base + added as u8 - 1) % 9 + 1
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

        add_strange_mod(self.values[y * self.local_width + x], offx + offy)
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

struct DistanceMap {
    inner: Vec<usize>,
    width: usize,
}

impl DistanceMap {
    fn new(grid: &Grid) -> Self {
        let inner = vec![usize::MAX; grid.width() * grid.height()];
        Self {
            inner,
            width: grid.width(),
        }
    }

    fn get(&self, (x, y): (usize, usize)) -> usize {
        self.inner[y * self.width + x]
    }

    fn set(&mut self, (x, y): (usize, usize), value: usize) {
        self.inner[y * self.width + x] = value;
    }
}

fn shortest_path(grid: &Grid) -> Option<usize> {
    let start = (0, 0);
    let goal = (grid.width() - 1, grid.height() - 1);

    let mut dist = DistanceMap::new(grid);
    let mut heap = BinaryHeap::new();

    dist.set(start, 0);
    heap.push(Reverse(State {
        cost: 0,
        position: start,
    }));

    while let Some(Reverse(State { cost, position })) = heap.pop() {
        if position == goal {
            return Some(cost);
        }

        if cost > dist.get(position) {
            continue;
        }

        let (px, py) = position;
        for neighbor in grid.get_neighbors(px, py) {
            let next = State {
                cost: cost + grid.get(neighbor.0, neighbor.1) as usize,
                position: neighbor,
            };

            if next.cost < dist.get(next.position) {
                heap.push(Reverse(next));
                dist.set(next.position, next.cost);
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
        shortest_path(&grid).unwrap()
    }

    fn part2(input: &PuzzleInput) -> usize {
        let grid = Grid::new(input, 5);
        shortest_path(&grid).unwrap()
    }
}
