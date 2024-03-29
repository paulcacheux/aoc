use crate::aoc2023::Aoc2023;
use crate::traits::days::Day9;
use crate::traits::ParseInput;
use crate::traits::Solution;

impl ParseInput<Day9> for Aoc2023 {
    type Parsed = Vec<Vec<i32>>;

    fn parse_input(input: &str) -> Self::Parsed {
        input
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|value| value.parse().unwrap())
                    .collect()
            })
            .collect()
    }
}

impl Solution<Day9> for Aoc2023 {
    type Part1Output = i32;
    type Part2Output = i32;

    fn part1(input: &Vec<Vec<i32>>) -> i32 {
        solve(input, false)
    }

    fn part2(input: &Vec<Vec<i32>>) -> i32 {
        solve(input, true)
    }
}

fn solve(input: &Vec<Vec<i32>>, part2: bool) -> i32 {
    let mut sum = 0;
    for line in input {
        let mut history = Vec::new();
        let mut current = line.clone();

        while !current.iter().all(|&v| v == 0) {
            history.push(
                *(if part2 {
                    current.first()
                } else {
                    current.last()
                })
                .unwrap(),
            );

            let mut next_line: Vec<i32> = Vec::with_capacity(current.len() - 1);
            for [a, b] in current.array_windows() {
                let delta = b - a;
                next_line.push(delta);
            }
            current = next_line;
        }

        sum += history.into_iter().rev().fold(
            0,
            |acc, value| {
                if part2 {
                    value - acc
                } else {
                    value + acc
                }
            },
        );
    }
    sum
}
