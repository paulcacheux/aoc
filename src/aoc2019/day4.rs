use crate::aoc2019::Aoc2019;
use advent_of_code_traits::days::Day4;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;

impl ParseInput<Day4> for Aoc2019 {
    type Parsed = (u32, u32);

    fn parse_input(input: &str) -> (u32, u32) {
        let mut iter = input.split('-');
        let start = iter.next().unwrap().parse().unwrap();
        let end = iter.next().unwrap().parse().unwrap();
        (start, end)
    }
}

fn is_valid_part1(n: &u32) -> bool {
    let digits: Vec<_> = n.to_string().bytes().map(|b| b - b'0').collect();
    let mut pair = false;
    for w in digits.windows(2) {
        if w[0] > w[1] {
            return false;
        }
        if w[0] == w[1] {
            pair = true
        }
    }
    pair
}

fn is_valid_part2(n: &u32) -> bool {
    let mut special_digits = vec![0];
    special_digits.extend(n.to_string().bytes().map(|b| b - b'0'));
    special_digits.push(10);

    let mut pair = false;

    for w in special_digits.windows(2) {
        if w[0] > w[1] {
            return false;
        }
    }

    for w in special_digits.windows(4) {
        if w[1] == w[2] && w[0] != w[1] && w[2] != w[3] {
            pair = true
        }
    }
    pair
}

impl Solution<Day4> for Aoc2019 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &(u32, u32)) -> usize {
        (input.0..=input.1).filter(is_valid_part1).count()
    }

    fn part2(input: &(u32, u32)) -> usize {
        (input.0..=input.1).filter(is_valid_part2).count()
    }
}
