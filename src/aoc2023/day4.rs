use ahash::HashSet;

use crate::aoc2023::Aoc2023;
use crate::traits::days::Day4;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug)]
pub struct Card {
    id: u32,
    winning: Vec<u32>,
    got: Vec<u32>,
}

impl ParseInput<Day4> for Aoc2023 {
    type Parsed = Vec<Card>;

    fn parse_input(input: &str) -> Self::Parsed {
        input
            .lines()
            .map(|line| {
                let (before, after) = line.split_at(line.find(": ").unwrap());
                let after = &after[2..];

                let id = before.strip_prefix("Card").unwrap().trim().parse().unwrap();
                let (winning, got) = after.split_at(after.find(" | ").unwrap());
                let got = &got[3..];

                let winning = winning
                    .split_ascii_whitespace()
                    .map(|value| value.trim().parse().unwrap())
                    .collect();
                let got = got
                    .split_ascii_whitespace()
                    .map(|value| value.trim().parse().unwrap())
                    .collect();

                Card { id, winning, got }
            })
            .collect()
    }
}

impl Solution<Day4> for Aoc2023 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Vec<Card>) -> u32 {
        let mut res = 0;
        for card in input {
            let winning: HashSet<_> = card.winning.iter().copied().collect();
            let got: HashSet<_> = card.got.iter().copied().collect();

            let inter = winning.intersection(&got).count();
            let points = if inter > 0 { 1 << (inter - 1) } else { 0 };
            res += points;
        }
        res
    }

    fn part2(input: &Vec<Card>) -> u32 {
        todo!()
    }
}
