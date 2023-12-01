use crate::aoc2023::Aoc2023;
use crate::traits::days::Day1;
use crate::traits::ParseInput;
use crate::traits::Solution;

impl ParseInput<Day1> for Aoc2023 {
    type Parsed = Vec<String>;

    fn parse_input(input: &str) -> Self::Parsed {
        input.lines().map(|line| line.trim().to_owned()).collect()
    }
}

const NEEDLES: &[(&str, &str, u32)] = &[
    ("1", "one", 1),
    ("2", "two", 2),
    ("3", "three", 3),
    ("4", "four", 4),
    ("5", "five", 5),
    ("6", "six", 6),
    ("7", "seven", 7),
    ("8", "eight", 8),
    ("9", "nine", 9),
];

fn solve(lines: &[String], with_letters: bool) -> u32 {
    let mut res = 0;
    for line in lines {
        let mut left = None;
        'left: for i in 0..line.len() {
            let sub = &line[i..];
            for (needle1, needle2, val) in NEEDLES {
                if sub.starts_with(needle1) || (with_letters && sub.starts_with(needle2)) {
                    left = Some(val);
                    break 'left;
                }
            }
        }

        let mut right = None;
        'right: for i in (0..line.len()).rev() {
            let sub = &line[..(i + 1)];
            for (needle1, needle2, val) in NEEDLES {
                if sub.ends_with(needle1) || (with_letters && sub.ends_with(needle2)) {
                    right = Some(val);
                    break 'right;
                }
            }
        }

        res += left.unwrap() * 10 + right.unwrap()
    }
    res
}

impl Solution<Day1> for Aoc2023 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(lines: &Vec<String>) -> u32 {
        solve(lines, false)
    }

    fn part2(lines: &Vec<String>) -> u32 {
        solve(lines, true)
    }
}
