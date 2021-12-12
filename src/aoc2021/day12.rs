use std::collections::HashMap;
use std::collections::HashSet;

use crate::aoc2021::Aoc2021;
use advent_of_code_traits::days::Day12;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;

use string_interner::DefaultSymbol;
use string_interner::StringInterner;

#[derive(Debug)]
pub struct PuzzleInput {
    pairs: Vec<(Node, Node)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    Start,
    End,
    Big(DefaultSymbol),
    Small(DefaultSymbol),
}

impl ParseInput<Day12> for Aoc2021 {
    type Parsed = PuzzleInput;

    fn parse_input(input: &str) -> PuzzleInput {
        let mut interner = StringInterner::new();
        fn part_parse(interner: &mut StringInterner, part: &str) -> Node {
            match part {
                "start" => Node::Start,
                "end" => Node::End,
                _ => {
                    let first = part.chars().next().unwrap();
                    if first.is_ascii_lowercase() {
                        Node::Small(interner.get_or_intern(part))
                    } else {
                        Node::Big(interner.get_or_intern(part))
                    }
                }
            }
        }

        let pairs = input
            .lines()
            .map(|line| {
                let mut parts = line.trim().split('-');
                let left = parts.next().unwrap();
                let right = parts.next().unwrap();
                (
                    part_parse(&mut interner, left),
                    part_parse(&mut interner, right),
                )
            })
            .collect();

        PuzzleInput { pairs }
    }
}

fn build_links(input: &Vec<(Node, Node)>) -> HashMap<Node, HashSet<Node>> {
    let mut links: HashMap<Node, HashSet<Node>> = HashMap::new();
    for (a, b) in input {
        links.entry(a.clone()).or_default().insert(b.clone());
        links.entry(b.clone()).or_default().insert(a.clone());
    }
    links
}

fn count_paths(
    links: &HashMap<Node, HashSet<Node>>,
    visited_builder: fn(&[Node]) -> HashSet<Node>,
) -> usize {
    let mut working_set = vec![vec![Node::Start]];
    let mut counter = 0;
    while let Some(current) = working_set.pop() {
        let last = &current[current.len() - 1];
        if last == &Node::End {
            counter += 1;
            continue;
        }

        let visited_set = visited_builder(&current);

        if let Some(linked) = links.get(last) {
            for next in linked {
                if !visited_set.contains(next) {
                    let mut list = current.clone();
                    list.push(next.clone());
                    working_set.push(list);
                }
            }
        }
    }
    counter
}

fn part1_visited_builder(current: &[Node]) -> HashSet<Node> {
    current
        .iter()
        .filter(|n| match n {
            Node::Big(_) => false,
            _ => true,
        })
        .cloned()
        .collect()
}

impl Solution<Day12> for Aoc2021 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &PuzzleInput) -> usize {
        let links = build_links(&input.pairs);
        count_paths(&links, part1_visited_builder)
    }

    fn part2(input: &PuzzleInput) -> usize {
        let links = build_links(&input.pairs);
        count_paths(&links, |current| {
            let mut set = HashSet::new();
            let mut counter: HashMap<DefaultSymbol, usize> = HashMap::new();
            for node in current {
                match node {
                    Node::Start | Node::End => {
                        set.insert(node.clone());
                    }
                    Node::Big(_) => {}
                    Node::Small(name) => {
                        set.insert(node.clone());
                        *counter.entry(*name).or_default() += 1;
                    }
                }
            }

            if !counter.values().any(|&c| c >= 2) {
                set.retain(|n| if let Node::Small(_) = n { false } else { true })
            }
            set
        })
    }
}
