use std::ops::RangeInclusive;

use itertools::Itertools;

use crate::aoc2022::grid::Grid;
use crate::aoc2022::Aoc2022;
use crate::traits::days::Day14;
use crate::traits::ParseInput;
use crate::traits::Solution;

impl ParseInput<Day14> for Aoc2022 {
    type Parsed = Vec<Vec<(u32, u32)>>;

    fn parse_input(input: &str) -> Self::Parsed {
        input
            .lines()
            .map(str::trim)
            .map(|line| {
                line.split(" -> ")
                    .map(|pairs| {
                        pairs
                            .split(',')
                            .map(|coord| coord.parse().unwrap())
                            .collect_tuple()
                            .unwrap()
                    })
                    .collect()
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Air,
    Rock,
    Sand,
}

const SAND_FOUNTAIN: (u32, u32) = (500, 0);

impl Solution<Day14> for Aoc2022 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Vec<Vec<(u32, u32)>>) -> u32 {
        let (minx, maxx, miny, maxy) = compute_min_max(input);
        let mut grid = Grid::new(
            (maxx - minx + 1) as usize,
            (maxy - miny + 1) as usize,
            Cell::Air,
        );

        fill_grid(&mut grid, input, minx, miny);

        let mut counter = 0;
        let fountain = (SAND_FOUNTAIN.0 - minx, SAND_FOUNTAIN.1 - miny);
        // we insert the sand starting from the last moving position
        // because the path from the foutain to this point is always the same
        // if we can't we default to the fountain
        let mut insertion_point = fountain;
        while let Some(sp) = insert_sand(&grid, insertion_point, fountain) {
            counter += 1;
            grid.set(sp.pos.0 as usize, sp.pos.1 as usize, Cell::Sand);
            insertion_point = sp.previous;
        }

        counter
    }

    fn part2(input: &Vec<Vec<(u32, u32)>>) -> u32 {
        let (mut minx, mut maxx, miny, mut maxy) = compute_min_max(input);

        // part 2 required 1 more layer
        // and a bit of space around
        maxy += 1;
        minx -= maxy;
        maxx += maxy;

        let mut grid = Grid::new(
            (maxx - minx + 1) as usize,
            (maxy - miny + 1) as usize,
            Cell::Air,
        );

        fill_grid(&mut grid, input, minx, miny);

        let mut counter = 1;
        grid.set(
            (SAND_FOUNTAIN.0 - minx) as usize,
            (SAND_FOUNTAIN.1 - miny) as usize,
            Cell::Sand,
        );
        for y in 1..grid.height {
            for x in 0..grid.width {
                if *grid.get(x, y) != Cell::Air {
                    continue;
                }

                if [-1, 0, 1]
                    .into_iter()
                    .filter_map(|dx| offset_and_validate(grid.width, x as u32, dx))
                    .any(|nx| *grid.get(nx as usize, y - 1) == Cell::Sand)
                {
                    grid.set(x, y, Cell::Sand);
                    counter += 1;
                }
            }
        }
        counter
    }
}

fn range(start: u32, end: u32) -> RangeInclusive<u32> {
    if start <= end {
        start..=end
    } else {
        end..=start
    }
}

struct SandPoint {
    pos: (u32, u32),
    previous: (u32, u32),
}

fn insert_sand(grid: &Grid<Cell>, source: (u32, u32), fountain: (u32, u32)) -> Option<SandPoint> {
    let (mut sx, mut sy) = source;
    let (mut px, mut py) = fountain;

    let deltasx = [0, -1, 1];

    loop {
        let mut found = false;
        for &dx in &deltasx {
            let nx = offset_and_validate(grid.width, sx, dx)?;
            let ny = offset_and_validate(grid.height, sy, 1)?;

            if let Cell::Air = grid.get(nx as usize, ny as usize) {
                px = sx;
                py = sy;
                sx = nx;
                sy = ny;
                found = true;
                break;
            }
        }

        if !found {
            return Some(SandPoint {
                pos: (sx, sy),
                previous: (px, py),
            });
        }
    }
}

#[allow(dead_code)]
fn dbg_grid(grid: &Grid<Cell>) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            let cell = grid.get(x, y);
            print!(
                "{}",
                match cell {
                    Cell::Air => '.',
                    Cell::Rock => '#',
                    Cell::Sand => 'o',
                }
            )
        }
        println!()
    }
}

fn offset_and_validate(grid_dim: usize, base: u32, delta: i32) -> Option<u32> {
    let comb = base as i32 + delta;
    if comb < 0 || comb as usize >= grid_dim {
        None
    } else {
        Some(comb as u32)
    }
}

fn compute_min_max(input: &[Vec<(u32, u32)>]) -> (u32, u32, u32, u32) {
    let mut minx = u32::MAX;
    let mut maxx = 0;
    let mut miny = u32::MAX;
    let mut maxy = 0;
    for points in input.iter().chain([&vec![SAND_FOUNTAIN]]) {
        for &(x, y) in points {
            if x < minx {
                minx = x;
            }
            if x > maxx {
                maxx = x;
            }
            if y < miny {
                miny = y;
            }
            if y > maxy {
                maxy = y;
            }
        }
    }
    (minx, maxx, miny, maxy)
}

fn fill_grid(grid: &mut Grid<Cell>, input: &[Vec<(u32, u32)>], minx: u32, miny: u32) {
    for walls in input {
        for [(sx, sy), (ex, ey)] in walls.array_windows::<2>() {
            for x in range(*sx, *ex) {
                for y in range(*sy, *ey) {
                    let x = x - minx;
                    let y = y - miny;
                    grid.set(x as usize, y as usize, Cell::Rock);
                }
            }
        }
    }
}
