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

fn compute_first_index<const SIZE: usize>(input: &[u8]) -> usize {
    let mut start = 0;
    'main: while start < (input.len() - SIZE) {
        for i in 0..SIZE {
            for j in (i + 1)..SIZE {
                if input[start + i] == input[start + j] {
                    start += i + 1; // skip all the repetitive checks
                    continue 'main;
                }
            }
        }

        return start + SIZE;
    }

    unreachable!()
}

impl Solution<Day6> for Aoc2022 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Vec<u8>) -> usize {
        compute_first_index::<4>(input)
    }

    fn part2(input: &Vec<u8>) -> usize {
        compute_first_index::<14>(input)
    }
}
