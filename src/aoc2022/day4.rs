use crate::aoc2022::Aoc2022;
use advent_of_code_traits::days::Day4;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;
use itertools::Itertools;

type Pair = (u32, u32);

impl ParseInput<Day4> for Aoc2022 {
    type Parsed = Vec<(Pair, Pair)>;

    fn parse_input(input: &str) -> Self::Parsed {
        input
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|pair| {
                        pair.split('-')
                            .map(|d| d.parse().unwrap())
                            .collect_tuple::<Pair>()
                            .unwrap()
                    })
                    .collect_tuple()
                    .unwrap()
            })
            .collect()
    }
}

impl Solution<Day4> for Aoc2022 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Vec<(Pair, Pair)>) -> u32 {
        let mut res = 0;
        for (left, right) in input {
            if left.0 >= right.0 && left.1 <= right.1 || right.0 >= left.0 && right.1 <= left.1 {
                res += 1;
            }
        }
        res
    }

    fn part2(input: &Vec<(Pair, Pair)>) -> u32 {
        let mut res = 0;
        for (left, right) in input {
            if left.0 >= right.0 && left.0 <= right.1 || right.0 >= left.0 && right.0 <= left.1 {
                res += 1;
            }
        }
        res
    }
}
