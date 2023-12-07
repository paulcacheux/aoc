use std::collections::HashMap;

use crate::aoc2023::Aoc2023;
use crate::traits::days::Day7;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Hand<const J: bool> {
    cards: [u8; 5],
}

impl Hand<false> {
    fn into_joker(self) -> Hand<true> {
        Hand::<true> { cards: self.cards }
    }
}

#[inline]
fn shift_signature(input: &mut [u8]) {
    for i in 1..input.len() {
        if input[i] > 0 {
            input[i - 1] += 1;
            input[i] -= 1;
            break;
        }
    }
}

impl<const J: bool> Hand<J> {
    fn signature(&self) -> [u8; 5] {
        let mut values = HashMap::new();
        let mut joker = 0;
        for &card in &self.cards {
            if J && card == b'J' {
                joker += 1;
            } else {
                *values.entry(card).or_insert(0) += 1;
            }
        }
        let mut signature = [0; 5];
        for value in values.values() {
            signature[5 - value] += 1;
        }

        if J && joker != 0 {
            for _ in 0..joker {
                shift_signature(&mut signature);
            }
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
                b'9' => 9,
                b'8' => 8,
                b'7' => 7,
                b'6' => 6,
                b'5' => 5,
                b'4' => 4,
                b'3' => 3,
                b'2' => 2,
                _ => unreachable!(),
            };
        }
        zvalues
    }
}

impl<const J: bool> Ord for Hand<J> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.signature().cmp(&other.signature()) {
            std::cmp::Ordering::Equal => self.zvalues().cmp(&other.zvalues()),
            other => other,
        }
    }
}

impl<const J: bool> PartialOrd for Hand<J> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct Entry<const J: bool> {
    hand: Hand<J>,
    bet: u64,
}

impl ParseInput<Day7> for Aoc2023 {
    type Parsed = Vec<Entry<false>>;

    fn parse_input(input: &str) -> Self::Parsed {
        input
            .lines()
            .map(|line| {
                let mut iter = line.split_ascii_whitespace();
                let hand = iter
                    .next()
                    .unwrap()
                    .bytes()
                    .take(5)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                let bet = iter.next().unwrap().parse().unwrap();
                Entry {
                    hand: Hand { cards: hand },
                    bet,
                }
            })
            .collect()
    }
}

impl Solution<Day7> for Aoc2023 {
    type Part1Output = u64;
    type Part2Output = u64;

    fn part1(input: &Vec<Entry<false>>) -> u64 {
        let mut input = input.clone();
        input.sort_by_key(|entry| entry.hand);
        input
            .into_iter()
            .enumerate()
            .map(|(i, entry)| (i as u64 + 1) * entry.bet)
            .sum()
    }

    fn part2(input: &Vec<Entry<false>>) -> u64 {
        let mut input: Vec<_> = input
            .iter()
            .cloned()
            .map(|entry| Entry {
                hand: entry.hand.into_joker(),
                bet: entry.bet,
            })
            .collect();
        input.sort_by_key(|entry| entry.hand);
        input
            .into_iter()
            .enumerate()
            .map(|(i, entry)| (i as u64 + 1) * entry.bet)
            .sum()
    }
}
