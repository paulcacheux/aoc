use crate::aoc2021::Aoc2021;
use advent_of_code_traits::days::Day10;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;

#[derive(Debug, Clone, Copy)]
pub enum Element {
    LParen,
    LSquare,
    LCurly,
    LTriangle,
    RParen,
    RSquare,
    RCurly,
    RTriangle,
}

impl ParseInput<Day10> for Aoc2021 {
    type Parsed = Vec<Vec<Element>>;

    fn parse_input(input: &str) -> Vec<Vec<Element>> {
        input
            .lines()
            .map(|line| {
                line.trim()
                    .bytes()
                    .map(|b| match b {
                        b'(' => Element::LParen,
                        b'[' => Element::LSquare,
                        b'{' => Element::LCurly,
                        b'<' => Element::LTriangle,
                        b')' => Element::RParen,
                        b']' => Element::RSquare,
                        b'}' => Element::RCurly,
                        b'>' => Element::RTriangle,
                        _ => panic!("Unknown element"),
                    })
                    .collect()
            })
            .collect()
    }
}

enum LineState {
    Valid,
    Incomplete(Vec<Element>),
    CloseWithoutOpen,
    Invalid(Element),
}

fn investigate_line(line: &[Element]) -> LineState {
    let mut stack = Vec::new();

    for &element in line {
        match element {
            Element::LParen | Element::LSquare | Element::LCurly | Element::LTriangle => {
                stack.push(element);
            }
            other => match (stack.pop(), other) {
                (None, _) => return LineState::CloseWithoutOpen,
                (Some(Element::LParen), Element::RParen)
                | (Some(Element::LSquare), Element::RSquare)
                | (Some(Element::LCurly), Element::RCurly)
                | (Some(Element::LTriangle), Element::RTriangle) => {
                    // all good
                }
                _ => return LineState::Invalid(other),
            },
        }
    }

    if stack.is_empty() {
        LineState::Valid
    } else {
        LineState::Incomplete(stack)
    }
}

impl Solution<Day10> for Aoc2021 {
    type Part1Output = u32;
    type Part2Output = u64;

    fn part1(input: &Vec<Vec<Element>>) -> u32 {
        let mut score = 0;

        for line in input {
            match investigate_line(line) {
                LineState::Incomplete(_) | LineState::Valid | LineState::CloseWithoutOpen => {}
                LineState::Invalid(prob) => {
                    let current_score = match prob {
                        Element::RParen => 3,
                        Element::RSquare => 57,
                        Element::RCurly => 1197,
                        Element::RTriangle => 25137,
                        _ => 0,
                    };
                    score += current_score;
                }
            }
        }
        score
    }

    fn part2(input: &Vec<Vec<Element>>) -> u64 {
        let mut scores = Vec::new();
        for line in input {
            let line_score = match investigate_line(line) {
                LineState::Incomplete(mut stack) => {
                    let mut line_score = 0;
                    while let Some(elem) = stack.pop() {
                        let value = match elem {
                            Element::LParen => 1,
                            Element::LSquare => 2,
                            Element::LCurly => 3,
                            Element::LTriangle => 4,
                            _ => unreachable!(),
                        };
                        line_score = line_score * 5 + value;
                    }
                    Some(line_score)
                }
                _ => None,
            };

            scores.extend(line_score);
        }

        scores.sort_unstable();
        scores[scores.len() / 2]
    }
}
