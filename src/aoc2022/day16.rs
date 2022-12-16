use std::cmp::Ordering;
use std::collections::VecDeque;

use crate::aoc2022::Aoc2022;
use crate::traits::days::Day16;
use crate::traits::ParseInput;
use crate::traits::Solution;

use ahash::HashMap;
use regex::Regex;

use string_interner::DefaultBackend;
use string_interner::StringInterner;

type StringSymbol = string_interner::symbol::SymbolU16;
type StrInterner = StringInterner<DefaultBackend<StringSymbol>>;

#[derive(Debug, Clone)]
pub struct Input {
    aa_symbol: StringSymbol,
    valves: Vec<Valve>,
}

#[derive(Debug, Clone)]
pub struct Valve {
    name: StringSymbol,
    rate: u32,
    edges: Vec<StringSymbol>,
}

impl ParseInput<Day16> for Aoc2022 {
    type Parsed = Input;

    fn parse_input(input: &str) -> Self::Parsed {
        let line_re =
            Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ((\w|[, ])+)")
                .unwrap();

        let mut interner = StrInterner::new();

        let valves = input
            .lines()
            .map(str::trim)
            .map(|line| {
                let captures = line_re.captures(line).unwrap();
                let name = captures.get(1).unwrap().as_str().to_owned();
                let rate = captures.get(2).unwrap().as_str().parse().unwrap();
                let edges = captures
                    .get(3)
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .map(|s| interner.get_or_intern(s))
                    .collect();
                Valve {
                    name: interner.get_or_intern(name),
                    rate,
                    edges,
                }
            })
            .collect();

        let aa_symbol = interner.get_or_intern_static("AA");
        Input { aa_symbol, valves }
    }
}

impl Solution<Day16> for Aoc2022 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Input) -> u32 {
        let paths = solve_part1(&input.valves, 30, input.aa_symbol);
        paths.into_iter().map(|path| path.total_rate).max().unwrap()
    }

    fn part2(input: &Input) -> u32 {
        let paths: Vec<_> = solve_part1(&input.valves, 26, input.aa_symbol)
            .into_iter()
            .map(|p| {
                let mut p2 = PathPart2 {
                    nodes: p.nodes,
                    total_rate: p.total_rate,
                };
                p2.nodes.sort();
                p2
            })
            .collect();

        let mut max = 0;
        for a in &paths {
            for b in &paths {
                let rate = a.total_rate + b.total_rate;
                if rate > max && !intersect(&a.nodes, &b.nodes) {
                    max = rate;
                }
            }
        }
        max
    }
}

fn bfs(edges: &HashMap<StringSymbol, Valve>, start: StringSymbol) -> HashMap<StringSymbol, u32> {
    let mut open_queue = VecDeque::new();
    open_queue.push_back((start, 0));

    let mut visited = HashMap::default();

    while let Some((current, cost)) = open_queue.pop_front() {
        if current != start {
            visited.insert(current, cost);
        }

        let current = edges.get(&current).unwrap();
        for next_name in &current.edges {
            if !visited.contains_key(next_name) {
                open_queue.push_back((*next_name, cost + 1));
            }
        }
    }
    visited
}

#[derive(Debug)]
struct Path {
    nodes: Vec<StringSymbol>,
    last: StringSymbol,
    total_rate: u32,
    time: u32,
}

#[derive(Debug)]
struct PathPart2 {
    nodes: Vec<StringSymbol>,
    total_rate: u32,
}

fn solve_part1(input: &[Valve], steps: u32, aa_symbol: StringSymbol) -> Vec<Path> {
    let interesting_valve_names: Vec<_> = input
        .iter()
        .filter_map(|valve| {
            if valve.rate != 0 || valve.name == aa_symbol {
                Some(valve.name)
            } else {
                None
            }
        })
        .collect();

    let valves: HashMap<_, _> = input
        .iter()
        .map(|valve| (valve.name, valve.clone()))
        .collect();

    // bfs from all interesting points (and "AA") to other interesting points
    let mut costs = HashMap::default();
    for name in interesting_valve_names {
        costs.insert(
            name,
            bfs(&valves, name)
                .into_iter()
                .filter(|(name, _)| valves[name].rate != 0)
                .collect::<Vec<_>>(),
        );
    }

    // dfs
    let mut queue = vec![Path {
        nodes: Vec::new(),
        last: aa_symbol,
        total_rate: 0,
        time: 0,
    }];

    let mut paths = Vec::new();

    while let Some(current) = queue.pop() {
        let mut found_next = false;
        for (next, cost) in &costs[(&current.last)] {
            if current.nodes.contains(next) {
                continue;
            }

            let time = current.time + *cost + 1;
            if time > steps {
                continue;
            }

            found_next = true;
            let mut nodes = current.nodes.clone();
            nodes.push(*next);

            queue.push(Path {
                nodes,
                last: *next,
                total_rate: current.total_rate + valves[next].rate * (steps - time),
                time,
            });
        }

        // for test case, since we have the time to switch on all valves
        // we need to not check this boolean
        if !found_next {
            paths.push(current);
        }
    }
    paths
}

fn intersect(a: &[StringSymbol], b: &[StringSymbol]) -> bool {
    if a.is_empty() || b.is_empty() {
        return false;
    }

    let mut i = 0;
    let mut j = 0;

    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            Ordering::Less => i += 1,
            Ordering::Equal => return true,
            Ordering::Greater => j += 1,
        }
    }
    false
}
