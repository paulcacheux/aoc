use std::collections::HashSet;

use crate::aoc2022::grid::Direction;
use crate::aoc2022::grid::Grid;
use crate::aoc2023::Aoc2023;
use crate::traits::days::Day16;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    Slash,
    AntiSlash,
    Pipe,
    Dash,
}

impl ParseInput<Day16> for Aoc2023 {
    type Parsed = Grid<Cell>;

    fn parse_input(input: &str) -> Self::Parsed {
        Grid::parse(input, |c| match c {
            '.' => Cell::Empty,
            '/' => Cell::Slash,
            '\\' => Cell::AntiSlash,
            '|' => Cell::Pipe,
            '-' => Cell::Dash,
            _ => unreachable!(),
        })
    }
}

impl Solution<Day16> for Aoc2023 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Grid<Cell>) -> usize {
        compute_energy(input, 0, 0, Direction::East)
    }

    fn part2(input: &Grid<Cell>) -> usize {
        let mut best_energy = 0;

        for x in 0..input.width {
            let new = compute_energy(input, x, 0, Direction::South);
            if new > best_energy {
                best_energy = new;
            }

            let new = compute_energy(input, x, input.height - 1, Direction::North);
            if new > best_energy {
                best_energy = new;
            }
        }

        for y in 0..input.height {
            let new = compute_energy(input, 0, y, Direction::East);
            if new > best_energy {
                best_energy = new;
            }

            let new = compute_energy(input, input.width - 1, y, Direction::West);
            if new > best_energy {
                best_energy = new;
            }
        }

        best_energy
    }
}

fn compute_energy(grid: &Grid<Cell>, startx: usize, starty: usize, dir: Direction) -> usize {
    let mut open_queue = vec![(startx, starty, dir)];
    let mut visited = HashSet::new();
    let mut positions = Grid::new(grid.width, grid.height, false);

    while let Some((x, y, direction)) = open_queue.pop() {
        if !visited.insert((x, y, direction)) {
            continue;
        }
        positions.set(x, y, true);

        let cell: Cell = *grid.get(x, y);
        let next = match (cell, direction) {
            (Cell::Empty, dir) => Ok(dir),
            // /
            (Cell::Slash, Direction::East) => Ok(Direction::North),
            (Cell::Slash, Direction::West) => Ok(Direction::South),
            (Cell::Slash, Direction::North) => Ok(Direction::East),
            (Cell::Slash, Direction::South) => Ok(Direction::West),

            // \
            (Cell::AntiSlash, Direction::East) => Ok(Direction::South),
            (Cell::AntiSlash, Direction::West) => Ok(Direction::North),
            (Cell::AntiSlash, Direction::North) => Ok(Direction::West),
            (Cell::AntiSlash, Direction::South) => Ok(Direction::East),

            // |
            (Cell::Pipe, Direction::East | Direction::West) => {
                Err((Direction::North, Direction::South))
            }
            (Cell::Pipe, dir @ (Direction::North | Direction::South)) => Ok(dir),

            // -
            (Cell::Dash, Direction::North | Direction::South) => {
                Err((Direction::West, Direction::East))
            }
            (Cell::Dash, dir @ (Direction::East | Direction::West)) => Ok(dir),
        };

        match next {
            Ok(dir) => {
                maybe_append(grid, &mut open_queue, x, y, dir);
            }
            Err((dir1, dir2)) => {
                maybe_append(grid, &mut open_queue, x, y, dir1);
                maybe_append(grid, &mut open_queue, x, y, dir2);
            }
        };
    }

    positions.data.into_iter().filter(|&cell| cell).count()
}

fn dir_to_delta(dir: Direction) -> (isize, isize) {
    match dir {
        Direction::East => (1, 0),
        Direction::West => (-1, 0),
        Direction::North => (0, -1),
        Direction::South => (0, 1),
    }
}

#[inline]
fn maybe_append(
    grid: &Grid<Cell>,
    queue: &mut Vec<(usize, usize, Direction)>,
    x: usize,
    y: usize,
    dir: Direction,
) {
    let (dx, dy) = dir_to_delta(dir);
    let nx = x as isize + dx;
    let ny = y as isize + dy;

    if nx < 0 || ny < 0 {
        return;
    }

    let (nx, ny) = (nx as usize, ny as usize);
    if nx >= grid.width || ny >= grid.height {
        return;
    }

    queue.push((nx, ny, dir));
}
