use std::collections::VecDeque;

use crate::aoc2022::grid::Direction;
use crate::aoc2022::grid::Grid;
use crate::aoc2023::Aoc2023;
use crate::traits::days::Day17;
use crate::traits::ParseInput;
use crate::traits::Solution;

impl ParseInput<Day17> for Aoc2023 {
    type Parsed = Grid<u32>;

    fn parse_input(input: &str) -> Self::Parsed {
        Grid::parse(input, |c| c.to_digit(10).unwrap())
    }
}

impl Solution<Day17> for Aoc2023 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Grid<u32>) -> u32 {
        solve::<0, 3>(input)
    }

    fn part2(input: &Grid<u32>) -> u32 {
        solve::<4, 10>(input)
    }
}

fn solve<const MIN: usize, const MAX: usize>(grid: &Grid<u32>) -> u32 {
    let start = (0, 0);
    let mut best_costs: Grid<CostState<MAX>> = Grid::new(grid.width, grid.height, CostState::new());

    let mut open_queue = VecDeque::new();
    open_queue.push_back(CellState {
        position: start,
        cost: 0,
        current_dir: Direction::East,
        dir_count: 0,
    });

    while let Some(CellState {
        position: (cx, cy),
        cost,
        current_dir,
        dir_count,
    }) = open_queue.pop_front()
    {
        for (dir, npx, npy) in grid.get_neighbors_with_direction(cx, cy) {
            if Direction::are_opposite(current_dir, dir) {
                continue;
            }

            if current_dir != dir && dir_count < MIN {
                continue;
            }

            let next_dir_count = if current_dir == dir { dir_count + 1 } else { 1 };
            if next_dir_count > MAX {
                continue;
            }

            let total_cost = cost + *grid.get(npx, npy);
            if best_costs.get(npx, npy).get_cost(dir, next_dir_count) <= total_cost {
                continue;
            }

            open_queue.push_back(CellState {
                position: (npx, npy),
                cost: total_cost,
                current_dir: dir,
                dir_count: next_dir_count,
            });

            best_costs
                .get_mut(npx, npy)
                .set_cost(dir, next_dir_count, total_cost);
        }
    }

    best_costs
        .get(grid.width - 1, grid.height - 1)
        .get_min_cost()
}

#[derive(Debug, Clone, Copy)]
struct CellState {
    position: (usize, usize),
    cost: u32,
    current_dir: Direction,
    dir_count: usize,
}

#[derive(Debug, Clone)]
struct CostState<const MAX: usize> {
    costs: [[u32; MAX]; 4],
}

impl<const MAX: usize> CostState<MAX> {
    fn new() -> Self {
        Self {
            costs: [[u32::MAX; MAX]; 4],
        }
    }

    fn get_cost(&self, dir: Direction, dir_count: usize) -> u32 {
        self.costs[dir as usize][dir_count - 1]
    }

    fn get_min_cost(&self) -> u32 {
        let mut min = u32::MAX;
        for c in &self.costs {
            for &val in c {
                if val < min {
                    min = val;
                }
            }
        }
        min
    }

    fn set_cost(&mut self, dir: Direction, dir_count: usize, cost: u32) {
        self.costs[dir as usize][dir_count - 1] = cost;
    }
}
