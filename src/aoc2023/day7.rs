use std::collections::HashMap;

use crate::aoc2023::Aoc2023;
use crate::traits::days::Day7;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Hand {
    cards: [u8; 5],
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ComputedHand {
    sig: [u8; 5],
    zvalues: [u8; 5],
}

#[inline]
fn shift_signature(sig: &mut [u8]) {
    for i in 1..sig.len() {
        if sig[i] > 0 {
            sig[i - 1] += 1;
            sig[i] -= 1;
            return;
        }
    }
    sig[sig.len() - 1] += 1;
}

impl Hand {
    fn compute(&self, joker_mode: bool) -> ComputedHand {
        ComputedHand {
            sig: self.signature(joker_mode),
            zvalues: self.zvalues(joker_mode),
        }
    }

    fn signature(&self, joker_mode: bool) -> [u8; 5] {
        let mut values = HashMap::new();
        let mut joker = 0;
        for &card in &self.cards {
            if joker_mode && card == b'J' {
                joker += 1;
            } else {
                *values.entry(card).or_insert(0) += 1;
            }
        }
        let mut signature = [0; 5];
        for value in values.values() {
            signature[5 - value] += 1;
        }

        if joker_mode && joker != 0 {
            for _ in 0..joker {
                shift_signature(&mut signature);
            }
        }

        signature
    }

    fn zvalues(&self, joker_mode: bool) -> [u8; 5] {
        let mut zvalues = [0; 5];
        for (v, z) in self.cards.iter().zip(zvalues.iter_mut()) {
            *z = match *v {
                b'A' => 14,
                b'K' => 13,
                b'Q' => 12,
                b'J' => {
                    if joker_mode {
                        1
                    } else {
                        11
                    }
                }
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

impl Ord for ComputedHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.sig.cmp(&other.sig) {
            std::cmp::Ordering::Equal => self.zvalues.cmp(&other.zvalues),
            other => other,
        }
    }
}

impl PartialOrd for ComputedHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct Entry<H> {
    hand: H,
    bet: u64,
}

impl Entry<Hand> {
    fn compute(&self, joker_mode: bool) -> Entry<ComputedHand> {
        Entry {
            hand: self.hand.compute(joker_mode),
            bet: self.bet,
        }
    }
}

impl ParseInput<Day7> for Aoc2023 {
    type Parsed = Vec<Entry<Hand>>;

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

    fn part1(input: &Vec<Entry<Hand>>) -> u64 {
        solve(input, false)
    }

    fn part2(input: &Vec<Entry<Hand>>) -> u64 {
        solve(input, true)
    }
}

fn solve(input: &Vec<Entry<Hand>>, joker_mode: bool) -> u64 {
    let mut input: Vec<_> = input
        .iter()
        .map(|entry| entry.compute(joker_mode))
        .collect();
    input.sort_by_key(|entry| entry.hand);
    input
        .into_iter()
        .enumerate()
        .map(|(i, entry)| (i as u64 + 1) * entry.bet)
        .sum()
}
