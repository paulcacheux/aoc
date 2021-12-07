use advent_of_code_traits::{days::Day2, ParseInput, Solution};

use crate::aoc2021::Aoc2021;

#[derive(Debug)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

impl ParseInput<Day2> for Aoc2021 {
    type Parsed = Vec<(Direction, u32)>;

    fn parse_input(input: &str) -> Self::Parsed {
        let mut res = Vec::new();
        for line in input.lines() {
            let mut it = line.split_ascii_whitespace();
            let dir = it.next().unwrap();
            let dir = match dir {
                "forward" => Direction::Forward,
                "up" => Direction::Up,
                "down" => Direction::Down,
                _ => panic!("unknown direction"),
            };

            let count = it.next().unwrap();
            let count = count.parse().expect("failed to parse count");

            res.push((dir, count))
        }

        res
    }
}

impl Solution<Day2> for Aoc2021 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Vec<(Direction, u32)>) -> u32 {
        let mut horizontal = 0;
        let mut depth = 0;

        for (dir, count) in input {
            match dir {
                Direction::Forward => {
                    horizontal += count;
                }
                Direction::Up => {
                    depth -= count;
                }
                Direction::Down => {
                    depth += count;
                }
            }
        }

        horizontal * depth
    }

    fn part2(input: &Vec<(Direction, u32)>) -> u32 {
        let mut horizontal = 0;
        let mut depth = 0;
        let mut aim = 0;

        for (dir, count) in input {
            match dir {
                Direction::Forward => {
                    horizontal += count;
                    depth += aim * count;
                }
                Direction::Up => {
                    aim -= count;
                }
                Direction::Down => {
                    aim += count;
                }
            }
        }

        horizontal * depth
    }
}
