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
        let mut sum = 0;
        for line in input {
            let mut history = Vec::new();
            let mut current = line.clone();

            while !current.iter().all(|&v| v == 0) {
                history.push(current.clone());
                let mut next_line: Vec<i32> = Vec::with_capacity(current.len() - 1);
                for [a, b] in current.array_windows() {
                    let delta = b - a;
                    next_line.push(delta);
                }
                current = next_line;
            }

            for i in (0..history.len()).rev() {
                let under_value = if i + 1 < history.len() {
                    *history[i+1].last().unwrap()
                } else {
                    0
                };
                let new_value = history[i].last().unwrap() + under_value;
                history[i].push(new_value);
            }
            sum += history[0].last().unwrap();
        }
        sum
    }

    fn part2(input: &Vec<Vec<i32>>) -> i32 {
        let mut sum = 0;
        for line in input {
            let mut history = Vec::new();
            let mut current = line.clone();

            while !current.iter().all(|&v| v == 0) {
                history.push(current.clone());
                let mut next_line: Vec<i32> = Vec::with_capacity(current.len() - 1);
                for [a, b] in current.array_windows() {
                    let delta = b - a;
                    next_line.push(delta);
                }
                current = next_line;
            }

            for i in (0..history.len()).rev() {
                let under_value = if i + 1 < history.len() {
                    *history[i+1].first().unwrap()
                } else {
                    0
                };
                let new_value = history[i].first().unwrap() - under_value;

                let mut new_line = vec![new_value];
                new_line.extend_from_slice(&history[i]);
                history[i] = new_line;
            }
            sum += history[0].first().unwrap();
        }
        sum
    }
}
