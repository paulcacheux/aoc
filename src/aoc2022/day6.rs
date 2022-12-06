use crate::aoc2022::Aoc2022;
use advent_of_code_traits::days::Day6;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;

impl ParseInput<Day6> for Aoc2022 {
    type Parsed = Vec<u8>;

    fn parse_input(input: &str) -> Self::Parsed {
        input.trim().bytes().collect()
    }
}

fn compute_first_index(input: &[u8], size: usize) -> usize {
    'main: for (i, win) in input.windows(size).enumerate() {
        for i in 0..size {
            for j in (i + 1)..size {
                if win[i] == win[j] {
                    continue 'main;
                }
            }
        }
        return i + size;
    }
    unreachable!()
}

impl Solution<Day6> for Aoc2022 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Vec<u8>) -> usize {
        compute_first_index(input, 4)
    }

    fn part2(input: &Vec<u8>) -> usize {
        compute_first_index(input, 14)
    }
}
