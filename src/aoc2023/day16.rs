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
    type Part2Output = u32;

    fn part1(input: &Grid<Cell>) -> usize {
        compute_energy(input, 0, 0, Direction::East)
    }

    fn part2(_input: &Grid<Cell>) -> u32 {
        todo!()
    }
}

fn compute_energy(grid: &Grid<Cell>, startx: usize, starty: usize, dir: Direction) -> usize {
    let mut open_queue = vec![(startx, starty, dir)];
    let mut visited = HashSet::new();
    let mut positions = HashSet::new();

    while let Some((x, y, direction)) = open_queue.pop() {
        if !visited.insert((x, y, direction)) {
            continue;
        }
        positions.insert((x, y));

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

        let next_dir = match next {
            Ok(dir) => vec![dir],
            Err((dir1, dir2)) => vec![dir1, dir2],
        };

        for dir in next_dir {
            let (dx, dy) = dir_to_delta(dir);
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx < 0 || ny < 0 {
                continue;
            }

            let (nx, ny) = (nx as usize, ny as usize);
            if nx >= grid.width || ny >= grid.height {
                continue;
            }

            open_queue.push((nx, ny, dir));
        }
    }
    positions.len()
}

fn dir_to_delta(dir: Direction) -> (isize, isize) {
    match dir {
        Direction::East => (1, 0),
        Direction::West => (-1, 0),
        Direction::North => (0, -1),
        Direction::South => (0, 1),
    }
}
