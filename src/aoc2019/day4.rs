use crate::aoc2019::Aoc2019;
use crate::traits::days::Day4;
use crate::traits::ParseInput;
use crate::traits::Solution;
use itertools::Itertools;

impl ParseInput<Day4> for Aoc2019 {
    type Parsed = (u32, u32);

    fn parse_input(input: &str) -> (u32, u32) {
        let mut iter = input.trim().split('-');
        let start = iter.next().unwrap().parse().unwrap();
        let end = iter.next().unwrap().parse().unwrap();
        (start, end)
    }
}

fn is_valid_part1(n: &u32) -> bool {
    let nstr = n.to_string();
    let digits = nstr.bytes().map(|b| b - b'0');
    let mut pair = false;
    for (a, b) in digits.tuple_windows() {
        if a > b {
            return false;
        }
        if a == b {
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

    for (a, b) in special_digits.iter().copied().tuple_windows() {
        if a > b {
            return false;
        }
    }

    for (a, b, c, d) in special_digits.iter().copied().tuple_windows() {
        if b == c && a != b && c != d {
            pair = true
        }
    }
    pair
}

fn common_count(start: u32, end: u32, filter: fn(&u32) -> bool) -> usize {
    (start..=end).filter(filter).count()
}

impl Solution<Day4> for Aoc2019 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &(u32, u32)) -> usize {
        common_count(input.0, input.1, is_valid_part1)
    }

    fn part2(input: &(u32, u32)) -> usize {
        common_count(input.0, input.1, is_valid_part2)
    }
}
