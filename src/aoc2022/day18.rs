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
        let cube = Cube3D::from_input(input);
        solve_part1(&cube)
    }

    fn part2(input: &Vec<(u32, u32, u32)>) -> usize {
        let cube = Cube3D::from_input(input);

        assert!(!cube.get(0, 0, 0));

        let mut flood_filled = Cube3D::new(cube.width, true);
        let mut queue = vec![(0, 0, 0)];

        while let Some((x, y, z)) = queue.pop() {
            flood_filled.set(x, y, z, false);

            for (nx, ny, nz) in neighbors(x, y, z, cube.width) {
                if flood_filled.get(nx, ny, nz) && !cube.get(nx, ny, nz) {
                    queue.push((nx, ny, nz));
                }
            }
        }

        solve_part1(&flood_filled)
    }
}

fn solve_part1(cube: &Cube3D) -> usize {
    let mut counter = 0;
    for x in 0..cube.width {
        for y in 0..cube.width {
            for z in 0..cube.width {
                if !cube.get(x, y, z) {
                    continue;
                }

                let mut side_counter = 6;
                for (nx, ny, nz) in neighbors(x, y, z, cube.width) {
                    side_counter -= 1;
                    if !cube.get(nx, ny, nz) {
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
    std::iter::from_coroutine(
        #[coroutine]
        move || {
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
        },
    )
}

struct Cube3D {
    items: Vec<bool>,
    width: usize,
}

impl Cube3D {
    fn from_input(input: &[(u32, u32, u32)]) -> Self {
        let width = input.iter().flat_map(|t| [t.0, t.1, t.2]).max().unwrap() + 1;
        let width = width as usize;

        let mut cube = Cube3D::new(width, false);
        for &(x, y, z) in input {
            cube.set(x as usize, y as usize, z as usize, true);
        }
        cube
    }

    fn new(width: usize, val: bool) -> Self {
        Cube3D {
            items: vec![val; width * width * width],
            width,
        }
    }

    fn get(&self, x: usize, y: usize, z: usize) -> bool {
        let w = self.width;
        self.items[z * w * w + y * w + x]
    }

    fn set(&mut self, x: usize, y: usize, z: usize, val: bool) {
        let w = self.width;
        self.items[z * w * w + y * w + x] = val;
    }
}
