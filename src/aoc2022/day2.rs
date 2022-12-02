use crate::aoc2022::Aoc2022;
use advent_of_code_traits::days::Day2;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn from_str(s: &str) -> Self {
        match s {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => unreachable!("Wrong input"),
        }
    }

    fn shape_score(self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn compare(self, other: Self) -> Ordering {
        match (self, other) {
            (Hand::Rock, Hand::Rock) => Ordering::Equal,
            (Hand::Rock, Hand::Paper) => Ordering::Less,
            (Hand::Rock, Hand::Scissors) => Ordering::Greater,
            (Hand::Paper, Hand::Rock) => Ordering::Greater,
            (Hand::Paper, Hand::Paper) => Ordering::Equal,
            (Hand::Paper, Hand::Scissors) => Ordering::Less,
            (Hand::Scissors, Hand::Rock) => Ordering::Less,
            (Hand::Scissors, Hand::Paper) => Ordering::Greater,
            (Hand::Scissors, Hand::Scissors) => Ordering::Equal,
        }
    }

    fn part2_target_hand(left: Hand, right: Hand) -> Hand {
        let target_ordering = match right {
            Hand::Rock => Ordering::Greater,
            Hand::Paper => Ordering::Equal,
            Hand::Scissors => Ordering::Less,
        };

        match (left, target_ordering) {
            (Hand::Rock, Ordering::Less) => Hand::Paper,
            (Hand::Rock, Ordering::Equal) => Hand::Rock,
            (Hand::Rock, Ordering::Greater) => Hand::Scissors,
            (Hand::Paper, Ordering::Less) => Hand::Scissors,
            (Hand::Paper, Ordering::Equal) => Hand::Paper,
            (Hand::Paper, Ordering::Greater) => Hand::Rock,
            (Hand::Scissors, Ordering::Less) => Hand::Rock,
            (Hand::Scissors, Ordering::Equal) => Hand::Scissors,
            (Hand::Scissors, Ordering::Greater) => Hand::Paper,
        }
    }

    fn score(left: Self, right: Self) -> u32 {
        let score = match Hand::compare(left, right) {
            Ordering::Less => 6,
            Ordering::Equal => 3,
            Ordering::Greater => 0,
        };
        score + right.shape_score()
    }
}

impl ParseInput<Day2> for Aoc2022 {
    type Parsed = Vec<(Hand, Hand)>;

    fn parse_input(input: &str) -> Self::Parsed {
        let mut res = Vec::new();
        for line in input.lines() {
            let pair = line
                .split_whitespace()
                .take(2)
                .map(Hand::from_str)
                .collect_tuple()
                .unwrap();
            res.push(pair);
        }
        res
    }
}

impl Solution<Day2> for Aoc2022 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Vec<(Hand, Hand)>) -> u32 {
        let mut total_score = 0;
        for &(left, right) in input {
            total_score += Hand::score(left, right);
        }
        total_score
    }

    fn part2(input: &Vec<(Hand, Hand)>) -> u32 {
        let mut total_score = 0;
        for &(left, right) in input {
            let actual_right = Hand::part2_target_hand(left, right);
            total_score += Hand::score(left, actual_right);
        }
        total_score
    }
}
