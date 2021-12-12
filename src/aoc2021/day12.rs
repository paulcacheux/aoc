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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

fn build_links(input: &[(Node, Node)]) -> HashMap<Node, HashSet<Node>> {
    let mut links: HashMap<Node, HashSet<Node>> = HashMap::new();
    for &(a, b) in input {
        links.entry(a).or_default().insert(b);
        links.entry(b).or_default().insert(a);
    }
    links
}

#[derive(Debug, Clone)]
struct PathState<VS: VisitedState> {
    queue: Node,
    visited_state: VS,
}

impl<VS: VisitedState> PathState<VS> {
    fn new() -> Self {
        let mut visited_set = HashSet::new();
        visited_set.insert(Node::Start);
        PathState {
            queue: Node::Start,
            visited_state: VS::new(),
        }
    }

    fn is_full(&self) -> bool {
        self.queue == Node::End
    }

    fn append(&mut self, node: Node) {
        self.visited_state.append(node);
        self.queue = node;
    }
}

trait VisitedState: Clone {
    fn new() -> Self;
    fn can_go_to(&self, target: &Node) -> bool;
    fn append(&mut self, node: Node);
}

#[derive(Debug, Clone)]
struct Part1State {
    visited_set: HashSet<Node>,
}

impl VisitedState for Part1State {
    fn new() -> Self {
        let mut visited_set = HashSet::new();
        visited_set.insert(Node::Start);
        Part1State { visited_set }
    }

    fn can_go_to(&self, target: &Node) -> bool {
        !self.visited_set.contains(target)
    }

    fn append(&mut self, node: Node) {
        match node {
            Node::Start | Node::End | Node::Small(_) => {
                self.visited_set.insert(node);
            }
            Node::Big(_) => {}
        }
    }
}

#[derive(Debug, Clone)]
struct Part2State {
    visited_set: HashSet<Node>,
    smalls_visited: HashSet<DefaultSymbol>,
    double_checked: bool,
}

impl VisitedState for Part2State {
    fn new() -> Self {
        let mut visited_set = HashSet::new();
        visited_set.insert(Node::Start);
        Part2State {
            visited_set,
            smalls_visited: HashSet::default(),
            double_checked: false,
        }
    }

    fn can_go_to(&self, target: &Node) -> bool {
        !self.visited_set.contains(target)
    }

    fn append(&mut self, node: Node) {
        match node {
            Node::Start | Node::End => {
                self.visited_set.insert(node);
            }
            Node::Small(name) => {
                if self.double_checked {
                    self.visited_set.insert(node);
                } else if self.smalls_visited.contains(&name) {
                    self.visited_set
                        .extend(self.smalls_visited.drain().map(Node::Small));
                    self.double_checked = true;
                } else {
                    self.smalls_visited.insert(name);
                }
            }
            Node::Big(_) => {}
        }
    }
}

fn count_paths<VS: VisitedState>(links: &HashMap<Node, HashSet<Node>>) -> usize {
    let mut working_set = vec![PathState::<VS>::new()];
    let mut counter = 0;
    while let Some(current) = working_set.pop() {
        if current.is_full() {
            counter += 1;
            continue;
        }

        if let Some(linked) = links.get(&current.queue) {
            for next in linked {
                if current.visited_state.can_go_to(next) {
                    let mut state = current.clone();
                    state.append(*next);
                    working_set.push(state);
                }
            }
        }
    }
    counter
}

impl Solution<Day12> for Aoc2021 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &PuzzleInput) -> usize {
        let links = build_links(&input.pairs);
        count_paths::<Part1State>(&links)
    }

    fn part2(input: &PuzzleInput) -> usize {
        let links = build_links(&input.pairs);
        count_paths::<Part2State>(&links)
    }
}
