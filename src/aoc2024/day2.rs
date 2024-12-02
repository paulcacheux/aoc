use itertools::Itertools;

use crate::aoc2024::Aoc2024;
use crate::traits::days::Day2;
use crate::traits::ParseInput;
use crate::traits::Solution;

impl ParseInput<Day2> for Aoc2024 {
    type Parsed = Vec<Vec<u32>>;

    fn parse_input(input: &str) -> Self::Parsed {
        let mut grid = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for num in line.split_whitespace() {
                row.push(num.parse().unwrap());
            }
            grid.push(row);
        }
        grid
    }
}

impl Solution<Day2> for Aoc2024 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Vec<Vec<u32>>) -> u32 {
        let mut safe = 0;
        for line in input {
            if safe_line(line) {
                safe += 1;
            }
        }
        safe
    }

    fn part2(input: &Vec<Vec<u32>>) -> u32 {
        let mut safe = 0;
        for line in input {
            if safe_line(line) {
                safe += 1;
                continue;
            }

            for i in 0..line.len() {
                let left = &line[..i];
                let right = &line[i + 1..];
                if safe_line2(left, right) {
                    safe += 1;
                    break;
                }
            }
        }
        safe
    }
}

fn safe_line(line: &[u32]) -> bool {
    safe_line2(line, &[])
}

fn safe_line2(left: &[u32], right: &[u32]) -> bool {
    let mut ordering = None;

    for (&l, &r) in left.iter().chain(right).tuple_windows() {
        let diff = l.abs_diff(r);
        if !(1 <= diff && diff <= 3) {
            return false;
        }

        let current_ordering = l.cmp(&r);
        if let Some(ordering) = ordering {
            if ordering != current_ordering {
                return false;
            }
        }
        ordering = Some(current_ordering);
    }

    true
}
