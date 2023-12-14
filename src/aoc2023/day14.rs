use std::collections::HashMap;

use crate::aoc2022::grid::Grid;
use crate::aoc2023::Aoc2023;
use crate::traits::days::Day14;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Cell {
    Cube,
    Round,
    Empty,
}

impl ParseInput<Day14> for Aoc2023 {
    type Parsed = Grid<Cell>;

    fn parse_input(input: &str) -> Self::Parsed {
        Grid::parse(input, |c| match c {
            '#' => Cell::Cube,
            'O' => Cell::Round,
            '.' => Cell::Empty,
            _ => unreachable!(),
        })
    }
}

impl Solution<Day14> for Aoc2023 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Grid<Cell>) -> usize {
        let mut grid = input.clone();
        slide_north(&mut grid);

        compute_load(&grid)
    }

    fn part2(input: &Grid<Cell>) -> usize {
        let mut grid = input.clone();
        let mut cache: HashMap<Grid<Cell>, usize> = HashMap::new();

        let target = 1000000000;

        let mut step = 0;
        let mut shortcut_done = false;
        while step < target {
            if !shortcut_done {
                if let Some(old_step) = cache.get(&grid) {
                    let delta = step - old_step;
                    let offset: usize = (target - step) / delta * delta;
                    step += offset;
                    assert!(step < target);
                    shortcut_done = true;
                }
            }

            let start = grid.clone();
            slide_north(&mut grid);
            slide_west(&mut grid);
            slide_south(&mut grid);
            slide_east(&mut grid);

            if !shortcut_done {
                cache.insert(start, step);
            }
            step += 1;
        }

        compute_load(&grid)
    }
}

fn compute_load(grid: &Grid<Cell>) -> usize {
    grid.iter()
        .filter_map(|(_, y, cell)| match cell {
            Cell::Round => Some(grid.height - y),
            _ => None,
        })
        .sum()
}

fn slide<R1, R2, PM, NM>(
    grid: &mut Grid<Cell>,
    first_range: R1,
    second_range: R2,
    pos_mapper: PM,
    next_mapper: NM,
) where
    R1: Iterator<Item = usize>,
    R2: Iterator<Item = usize> + Clone,
    PM: Fn(usize, usize) -> (usize, usize),
    NM: Fn(usize, usize) -> Option<(usize, usize)>,
{
    for r1 in first_range {
        for r2 in second_range.clone() {
            let (x, y) = pos_mapper(r1, r2);
            if *grid.get(x, y) == Cell::Round {
                let (mut nx, mut ny) = (x, y);

                while let Some((cx, cy)) = next_mapper(nx, ny) {
                    if *grid.get(cx, cy) == Cell::Empty {
                        (nx, ny) = (cx, cy);
                    } else {
                        break;
                    }
                }

                if (nx, ny) != (x, y) {
                    grid.set(x, y, Cell::Empty);
                    grid.set(nx, ny, Cell::Round);
                }
            }
        }
    }
}

fn slide_north(grid: &mut Grid<Cell>) {
    slide(
        grid,
        0..grid.width,
        0..grid.height,
        |r1, r2| (r1, r2),
        |x, y| {
            if y > 0 {
                Some((x, y - 1))
            } else {
                None
            }
        },
    )
}

fn slide_west(grid: &mut Grid<Cell>) {
    slide(
        grid,
        0..grid.height,
        0..grid.width,
        |r1, r2| (r2, r1),
        |x, y| {
            if x > 0 {
                Some((x - 1, y))
            } else {
                None
            }
        },
    )
}

fn slide_south(grid: &mut Grid<Cell>) {
    let height = grid.height;
    slide(
        grid,
        0..grid.width,
        (0..grid.height).rev(),
        |r1, r2| (r1, r2),
        |x, y| {
            if y < (height - 1) {
                Some((x, y + 1))
            } else {
                None
            }
        },
    )
}

fn slide_east(grid: &mut Grid<Cell>) {
    let width = grid.width;
    slide(
        grid,
        0..grid.height,
        (0..grid.width).rev(),
        |r1, r2| (r2, r1),
        |x, y| {
            if x < width - 1 {
                Some((x + 1, y))
            } else {
                None
            }
        },
    )
}
