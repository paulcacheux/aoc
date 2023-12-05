use crate::aoc2023::Aoc2023;
use crate::traits::days::Day4;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug)]
pub struct Card {
    // no need for any id, the index of the card in the deck + 1 is the id
    winning: Vec<u32>,
    got: Vec<u32>,
}

impl Card {
    fn inter_count(&self) -> usize {
        let mut count = 0;
        for g in &self.got {
            if self.winning.contains(g) {
                count += 1;
            }
        }
        count
    }
}

impl ParseInput<Day4> for Aoc2023 {
    type Parsed = Vec<Card>;

    fn parse_input(input: &str) -> Self::Parsed {
        input
            .lines()
            .map(|line| {
                let (_, after) = line.split_at(line.find(": ").unwrap());
                let after = &after[2..];

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

                Card { winning, got }
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
            let inter = card.inter_count();
            let points = if inter > 0 { 1 << (inter - 1) } else { 0 };
            res += points;
        }
        res
    }

    fn part2(input: &Vec<Card>) -> u32 {
        let mut counts: Vec<u32> = vec![1; input.len()];
        for (id, card) in input.iter().enumerate() {
            let inter = card.inter_count();
            let current_count = counts[id];
            for i in 1..=inter {
                let next_id = id + i;
                counts[next_id] += current_count;
            }
        }

        counts.into_iter().sum()
    }
}
