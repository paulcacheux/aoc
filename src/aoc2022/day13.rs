use std::cmp::Ordering;

use crate::aoc2022::Aoc2022;
use crate::traits::days::Day13;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Item {
    Int(u32),
    List(Vec<Item>),
}

impl std::cmp::Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Item::Int(a), Item::Int(b)) => a.cmp(b),
            (a @ Item::Int(_), b @ Item::List(_)) => Item::List(vec![a.clone()]).cmp(b),
            (a @ Item::List(_), b @ Item::Int(_)) => a.cmp(&Item::List(vec![b.clone()])),
            (Item::List(a), Item::List(b)) => {
                for (ai, bi) in a.iter().zip(b) {
                    let ord = ai.cmp(bi);
                    if ord != Ordering::Equal {
                        return ord;
                    }
                }
                a.len().cmp(&b.len())
            }
        }
    }
}

impl std::cmp::PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
enum Token {
    Lhs,
    Rhs,
    Int(u32),
}

fn parse_line(line: &str) -> Item {
    let mut tokens = Vec::with_capacity(line.len());
    let mut current = None;
    for c in line.chars() {
        match c {
            '[' => tokens.push(Token::Lhs),
            ']' => {
                if let Some(c) = current {
                    tokens.push(Token::Int(c));
                    current = None;
                }
                tokens.push(Token::Rhs);
            }
            ',' => {
                if let Some(c) = current {
                    tokens.push(Token::Int(c));
                    current = None;
                }
            }
            c if c.is_ascii_digit() => {
                current = Some(current.unwrap_or(0) * 10 + c.to_digit(10).unwrap())
            }
            _ => unreachable!(),
        }
    }

    let mut stack = Vec::with_capacity(tokens.len());
    for token in tokens {
        match token {
            Token::Int(i) => stack.push(Some(Item::Int(i))),
            Token::Lhs => stack.push(None),
            Token::Rhs => {
                let mut index = stack.len() - 1;
                loop {
                    match &stack[index] {
                        None => break,
                        Some(_) => {}
                    }
                    index -= 1;
                }
                let list = Item::List(stack.drain(index + 1..).map(Option::unwrap).collect());
                stack.pop();
                stack.push(Some(list));
            }
        }
    }

    stack.pop().unwrap().unwrap()
}

impl ParseInput<Day13> for Aoc2022 {
    type Parsed = Vec<(Item, Item)>;

    fn parse_input(input: &str) -> Self::Parsed {
        input
            .lines()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .array_chunks::<2>()
            .map(|[left, right]| (parse_line(left), parse_line(right)))
            .collect()
    }
}

fn create_decoder_key(val: u32) -> Item {
    Item::List(vec![Item::List(vec![Item::Int(val)])])
}

impl Solution<Day13> for Aoc2022 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Vec<(Item, Item)>) -> usize {
        let mut res = 0;
        for (i, (a, b)) in input.iter().enumerate() {
            if a < b {
                res += i + 1;
            }
        }
        res
    }

    fn part2(input: &Vec<(Item, Item)>) -> usize {
        part2_index(2, input) * (part2_index(6, input) + 1)
    }
}

#[inline]
fn part2_index(val: u32, items: &[(Item, Item)]) -> usize {
    let key = create_decoder_key(val);
    let mut index = 1;
    for (a, b) in items {
        if a < &key {
            index += 1;
        }
        if b < &key {
            index += 1;
        }
    }
    index
}
