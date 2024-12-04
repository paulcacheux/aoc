use itertools::Itertools;

use crate::aoc2024::Aoc2024;
use crate::grid::Grid;
use crate::traits::days::Day4;
use crate::traits::ParseInput;
use crate::traits::Solution;

impl ParseInput<Day4> for Aoc2024 {
    type Parsed = Grid<char>;

    fn parse_input(input: &str) -> Self::Parsed {
        Grid::parse(input, |c| c)
    }
}

impl Solution<Day4> for Aoc2024 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Grid<char>) -> u32 {
        let mut count = 0;
        for y in 0..input.height {
            for (x0, x1, x2, x3) in (0..input.width).into_iter().tuple_windows() {
                if is_valid(input, [(x0, y), (x1, y), (x2, y), (x3, y)]) {
                    count += 1;
                }
            }
        }

        for x in 0..input.width {
            for (y0, y1, y2, y3) in (0..input.height).into_iter().tuple_windows() {
                if is_valid(input, [(x, y0), (x, y1), (x, y2), (x, y3)]) {
                    count += 1;
                }
            }
        }

        for y in 0..input.height {
            'x: for x in 0..input.width {
                let mut points = [(0, 0); 4];
                for i in 0..4 {
                    points[i] = (x + i, y + i);
                }

                if is_valid(input, points) {
                    count += 1;
                }

                for i in 0..4 {
                    if input.height < points[i].1 {
                        continue 'x;
                    }
                    points[i].1 = input.height - points[i].1;
                }

                if is_valid(input, points) {
                    count += 1;
                }
            }
        }

        count
    }

    fn part2(input: &Grid<char>) -> u32 {
        todo!()
    }
}

fn is_valid(input: &Grid<char>, points: [(usize, usize); 4]) -> bool {
    let direct = ['X', 'M', 'A', 'S'];
    let reverse = ['S', 'A', 'M', 'X'];
    let mut chars = ['\0'; 4];

    for (i, (x, y)) in points.into_iter().enumerate() {
        if x >= input.width || y >= input.height {
            return false;
        }

        chars[i] = *input.get(x, y);
    }

    chars == direct || chars == reverse
}
