use std::collections::HashMap;

use crate::aoc2023::Aoc2023;
use crate::traits::days::Day7;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Hand {
    cards: [u8; 5],
}

impl Hand {
    fn signature(&self) -> [u8; 5] {
        let mut values = HashMap::new();
        for &card in &self.cards {
            *values.entry(card).or_insert(0) += 1;
        }
        let mut signature = [0; 5];
        for value in values.values() {
            signature[5 - value] += 1;
        }
        signature
    }

    fn zvalues(&self) -> [u8; 5] {
        let mut zvalues = [0; 5];
        for (v, z) in self.cards.iter().zip(zvalues.iter_mut()) {
            *z = match *v {
                b'A' => 14,
                b'K' => 13,
                b'Q' => 12,
                b'J' => 11,
                b'T' => 10,
                other => other
            };
        }
        zvalues
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.signature().cmp(&other.signature()) {
            std::cmp::Ordering::Equal => {
                self.zvalues().cmp(&other.zvalues())
            },
            other => other,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Debug, Clone)]
pub struct Entry {
    hand: Hand,
    bet: u64,
}

impl ParseInput<Day7> for Aoc2023 {
    type Parsed = Vec<Entry>;

    fn parse_input(input: &str) -> Self::Parsed {
        input.lines().map(|line| {
            let mut iter = line.split_ascii_whitespace();
            let hand = iter.next().unwrap().bytes().take(5).collect::<Vec<_>>().try_into().unwrap();
            let bet = iter.next().unwrap().parse().unwrap();
            Entry {
                hand: Hand { cards: hand },
                bet,
            }
        }).collect()
    }
}

impl Solution<Day7> for Aoc2023 {
    type Part1Output = u64;
    type Part2Output = u32;

    fn part1(input: &Vec<Entry>) -> u64 {
        let mut input = input.clone();
        input.sort_by_key(|entry| entry.hand);

        dbg!(&input);

        input.into_iter().enumerate().map(|(i, entry)| (i as u64 + 1) * entry.bet).sum()
    }

    fn part2(_input: &Vec<Entry>) -> u32 {
        todo!()
    }
}
