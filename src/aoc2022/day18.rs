use itertools::Itertools;

use crate::aoc2022::Aoc2022;
use crate::traits::days::Day18;
use crate::traits::ParseInput;
use crate::traits::Solution;

impl ParseInput<Day18> for Aoc2022 {
    type Parsed = Vec<(u32, u32, u32)>;

    fn parse_input(input: &str) -> Self::Parsed {
        input
            .lines()
            .map(str::trim)
            .map(|line| {
                line.split(',')
                    .map(|w| w.parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect()
    }
}

impl Solution<Day18> for Aoc2022 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Vec<(u32, u32, u32)>) -> usize {
        let width = input
            .iter()
            .map(|t| [t.0, t.1, t.2])
            .flatten()
            .max()
            .unwrap()
            + 1;
        let width = width as usize;

        let mut grid = vec![vec![vec![false; width]; width]; width];

        for &(x, y, z) in input {
            grid[x as usize][y as usize][z as usize] = true;
        }

        solve_part1(&grid, width)
    }

    fn part2(input: &Vec<(u32, u32, u32)>) -> usize {
        let width = input
            .iter()
            .map(|t| [t.0, t.1, t.2])
            .flatten()
            .max()
            .unwrap()
            + 1;
        let width = width as usize;

        let mut grid = vec![vec![vec![false; width]; width]; width];
        for &(x, y, z) in input {
            grid[x as usize][y as usize][z as usize] = true;
        }

        assert!(!grid[0][0][0]);

        let mut flood_filled = vec![vec![vec![true; width]; width]; width];
        let mut queue = vec![(0, 0, 0)];

        while let Some((x, y, z)) = queue.pop() {
            flood_filled[x][y][z] = false;

            for (nx, ny, nz) in neighbors(x, y, z, width) {
                if flood_filled[nx][ny][nz] && !grid[nx][ny][nz] {
                    queue.push((nx, ny, nz));
                }
            }
        }

        solve_part1(&flood_filled, width)
    }
}

fn solve_part1(grid: &[Vec<Vec<bool>>], width: usize) -> usize {
    let mut counter = 0;
    for x in 0..width {
        for y in 0..width {
            for z in 0..width {
                if !grid[x][y][z] {
                    continue;
                }

                let mut side_counter = 6;
                for (nx, ny, nz) in neighbors(x, y, z, width) {
                    side_counter -= 1;
                    if !grid[nx][ny][nz] {
                        counter += 1;
                    }
                }
                counter += side_counter;
            }
        }
    }
    counter
}

#[inline]
fn neighbors(
    x: usize,
    y: usize,
    z: usize,
    width: usize,
) -> impl Iterator<Item = (usize, usize, usize)> {
    std::iter::from_generator(move || {
        if x > 0 {
            yield (x - 1, y, z);
        }
        if x + 1 < width {
            yield (x + 1, y, z);
        }
        if y > 0 {
            yield (x, y - 1, z);
        }
        if y + 1 < width {
            yield (x, y + 1, z);
        }
        if z > 0 {
            yield (x, y, z - 1);
        }
        if z + 1 < width {
            yield (x, y, z + 1);
        }
    })
}
