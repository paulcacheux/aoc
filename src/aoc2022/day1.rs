use crate::aoc2022::Aoc2022;
use advent_of_code_traits::days::Day1;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;

impl ParseInput<Day1> for Aoc2022 {
    type Parsed = Vec<Vec<u32>>;

    fn parse_input(input: &str) -> Self::Parsed {
        let mut res = vec![vec![]];

        for line in input.lines() {
            let line = line.trim();

            if let Ok(value) = line.parse() {
                res.last_mut().unwrap().push(value);
            } else {
                res.push(Vec::new())
            }
        }

        res
    }
}

impl Solution<Day1> for Aoc2022 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Vec<Vec<u32>>) -> u32 {
        input.into_iter().map(|elf| {
            elf.iter().sum()
        }).max().unwrap()
    }

    fn part2(input: &Vec<Vec<u32>>) -> u32 {
        let mut sums: Vec<u32> = input.into_iter().map(|elf| {
            elf.iter().sum()
        }).collect();

        sums.sort();
        sums.reverse();

        sums.into_iter().take(3).sum()
    }
}
