use std::cmp::Ordering;
use std::collections::HashMap;

use crate::aoc2023::Aoc2023;
use crate::traits::days::Day19;
use crate::traits::ParseInput;
use crate::traits::Solution;

pub struct Input {
    workflows: HashMap<String, Vec<Rule>>,
    ratings: Vec<Rating>,
}

#[derive(Debug, Default)]
struct Rating {
    values: [u64; 4],
}

#[derive(Debug)]
enum Rule {
    Filtering {
        field: usize,
        operator: std::cmp::Ordering,
        value: u64,
        target: String,
    },
    Direct {
        target: String,
    }
}

impl ParseInput<Day19> for Aoc2023 {
    type Parsed = Input;

    fn parse_input(input: &str) -> Self::Parsed {
        let mut in_ratings = false;
        let mut ratings = Vec::new();
        let mut workflows = HashMap::new();

        for line in input.lines() {
            if line.trim().is_empty() {
                in_ratings = true;
                continue;
            }

            if in_ratings {
                let line = &line[1..line.len() - 1];
                let mut rating = Rating::default();
                for group in line.split(',') {
                    let (field, value) = group.split_once('=').unwrap();
                    let value = value.parse().unwrap();
                    let index = match field {
                        "x" => 0,
                        "m" => 1,
                        "a" => 2,
                        "s" => 3,
                        _ => unreachable!(),
                    };
                    rating.values[index] = value;
                }
                ratings.push(rating);
            } else {
                let (name, rules) = line.split_once('{').unwrap();
                let name = name.to_owned();
                let rules = &rules[..rules.len() - 1];

                let mut parsed_rules = Vec::new();

                for rule in rules.split(',') {
                    if let Some((eval, target)) = rule.split_once(':') {
                        let mut chars = eval.chars();
                        let field = match chars.next().unwrap() {
                            'x' => 0,
                            'm' => 1,
                            'a' => 2,
                            's' => 3,
                            _ => unreachable!(),
                        };
                        let operator = match chars.next().unwrap() {
                            '>' => Ordering::Greater,
                            '<' => Ordering::Less,
                            _ => unreachable!(),
                        };
                        let value = eval[2..].parse().unwrap();
                        parsed_rules.push(Rule::Filtering {
                            field,
                            operator,
                            value,
                            target: target.to_owned(),
                        });
                    } else {
                        parsed_rules.push(Rule::Direct {
                            target: rule.to_owned(),
                        });
                    }
                }
                workflows.insert(name, parsed_rules);
            }
        }

        Input { workflows, ratings }
    }
}

impl Solution<Day19> for Aoc2023 {
    type Part1Output = u64;
    type Part2Output = u64;

    fn part1(input: &Input) -> u64 {
        let mut score = 0;
        for rating in &input.ratings {
            let mut current = "in";

            'wf: loop {
                if current == "A" {
                    score += rating.values.iter().sum::<u64>();
                    break;
                } else if current == "R" {
                    break;
                }

                let workflow = input.workflows.get(current).unwrap();
                for rule in workflow {
                    match rule {
                        Rule::Direct { target } => {
                            current = target;
                            continue 'wf;
                        }
                        Rule::Filtering { field, operator, value, target } => {
                            if rating.values[*field].cmp(value) == *operator {
                                current = target;
                                continue 'wf;
                            }
                        }
                    }
                }
            }
        }
        score
    }

    fn part2(_input: &Input) -> u64 {
        todo!()
    }
}