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

            let deltas = compute_deltas(x, y, z, width);
            for (dx, dy, dz) in deltas {
                let nx = x.wrapping_add_signed(dx);
                let ny = y.wrapping_add_signed(dy);
                let nz = z.wrapping_add_signed(dz);
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

                let deltas = compute_deltas(x, y, z, width);

                counter += 6 - deltas.len();
                for (dx, dy, dz) in deltas {
                    if !grid[x.wrapping_add_signed(dx)][y.wrapping_add_signed(dy)]
                        [z.wrapping_add_signed(dz)]
                    {
                        counter += 1;
                    }
                }
            }
        }
    }
    counter
}

fn compute_deltas(x: usize, y: usize, z: usize, width: usize) -> Vec<(isize, isize, isize)> {
    let mut deltas = Vec::with_capacity(6);
    if x > 0 {
        deltas.push((-1, 0, 0));
    }
    if x + 1 < width {
        deltas.push((1, 0, 0));
    }
    if y > 0 {
        deltas.push((0, -1, 0));
    }
    if y + 1 < width {
        deltas.push((0, 1, 0));
    }
    if z > 0 {
        deltas.push((0, 0, -1));
    }
    if z + 1 < width {
        deltas.push((0, 0, 1));
    }
    deltas
}
