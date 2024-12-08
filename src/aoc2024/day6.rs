use std::collections::HashSet;

use crate::aoc2024::Aoc2024;
use crate::grid::Grid;
use crate::traits::days::Day6;
use crate::traits::ParseInput;
use crate::traits::Solution;

impl ParseInput<Day6> for Aoc2024 {
    type Parsed = (Grid<bool>, usize, usize);

    fn parse_input(input: &str) -> Self::Parsed {
        let mut sx = 0;
        let mut sy = 0;
        let grid = Grid::parse(input, |c| c);

        let mut bgrid = Grid::new(grid.width, grid.height, false);
        grid.iter().for_each(|(x, y, &c)| {
            let val = match c {
                '#' => true,
                '.' => false,
                '^' => {
                    sx = x;
                    sy = y;
                    false
                }
                _ => unreachable!(),
            };
            bgrid.set(x, y, val);
        });
        (bgrid, sx, sy)
    }
}

impl Solution<Day6> for Aoc2024 {
    type Part1Output = usize;
    type Part2Output = u32;

    fn part1((grid, sx, sy): &(Grid<bool>, usize, usize)) -> usize {
        const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

        let (mut x, mut y) = (*sx, *sy);
        let mut didx = 0;
        let mut positions = HashSet::new();

        loop {
            positions.insert((x, y));
            let (dx, dy) = DIRECTIONS[didx];
            if let Some((nx, ny)) = offset_pair(grid, x, y, dx, dy) {
                if *grid.get(nx, ny) {
                    didx = (didx + 1) % 4;
                } else {
                    x = nx;
                    y = ny;
                }
            } else {
                break;
            }
        }

        positions.len()
    }

    fn part2(_input: &(Grid<bool>, usize, usize)) -> u32 {
        todo!()
    }
}

fn offset_pair(grid: &Grid<bool>, x: usize, y: usize, dx: isize, dy: isize) -> Option<(usize, usize)> {
    let nx = offset(x, dx, grid.width);
    let ny = offset(y, dy,grid.height);

    match (nx, ny) {
        (Some(nx), Some(ny)) => Some((nx, ny)),
        _ => None,
    }
}

fn offset(base: usize, offset: isize, max: usize) -> Option<usize> {
    let res = base as isize + offset;

    if res < 0 || res >= max as isize {
        return None;
    }

    Some(res as usize)
}
