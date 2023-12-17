use std::collections::BinaryHeap;

use crate::aoc2023::Aoc2023;
use crate::grid::Direction;
use crate::grid::Grid;
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
        solve::<1, 3>(input)
    }

    fn part2(input: &Grid<u32>) -> u32 {
        solve::<4, 10>(input)
    }
}

fn solve<const MIN: usize, const MAX: usize>(grid: &Grid<u32>) -> u32 {
    let start = (0, 0);
    let mut best_costs: Grid<[u32; 4]> = Grid::new(grid.width, grid.height, [u32::MAX; 4]);
    best_costs.set(0, 0, [0; 4]);

    let mut open_queue = BinaryHeap::new();
    open_queue.push(CellState {
        position: start,
        cost: 0,
        current_dir: Direction::East,
    });
    open_queue.push(CellState {
        position: start,
        cost: 0,
        current_dir: Direction::South,
    });

    while let Some(CellState {
        position: (x, y),
        cost,
        current_dir,
    }) = open_queue.pop()
    {
        if (x, y) == (grid.width - 1, grid.height - 1) {
            return cost;
        }

        let new_dirs = match current_dir {
            Direction::North | Direction::South => {
                [(Direction::West, -1, 0), (Direction::East, 1, 0)]
            }
            Direction::East | Direction::West => {
                [(Direction::North, 0, -1), (Direction::South, 0, 1)]
            }
        };

        for (new_dir, dx, dy) in new_dirs {
            let mut total_cost = cost;
            for factor in 1..=MAX {
                let dx = dx * factor as isize;
                let dy = dy * factor as isize;

                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx < 0 || ny < 0 {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);

                if nx >= grid.width || ny >= grid.height {
                    continue;
                }

                total_cost += *grid.get(nx, ny);

                if factor < MIN {
                    continue;
                }

                if best_costs.get(nx, ny)[new_dir as usize] <= total_cost {
                    continue;
                }

                open_queue.push(CellState {
                    position: (nx, ny),
                    cost: total_cost,
                    current_dir: new_dir,
                });

                best_costs.get_mut(nx, ny)[new_dir as usize] = total_cost;
            }
        }
    }

    unimplemented!()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CellState {
    cost: u32,
    position: (usize, usize),
    current_dir: Direction,
}

// reverse Order, so that the binary heap pops the min instead of the max
impl PartialOrd for CellState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CellState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.position.cmp(&self.position))
            .then_with(|| other.current_dir.cmp(&self.current_dir))
    }
}
