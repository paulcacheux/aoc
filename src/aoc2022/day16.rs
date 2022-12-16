use std::collections::VecDeque;

use crate::aoc2022::Aoc2022;
use crate::traits::days::Day16;
use crate::traits::ParseInput;
use crate::traits::Solution;

use ahash::HashMap;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Valve {
    name: String,
    rate: u32,
    edges: Vec<String>,
}

impl ParseInput<Day16> for Aoc2022 {
    type Parsed = Vec<Valve>;

    fn parse_input(input: &str) -> Self::Parsed {
        let line_re =
            Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ((\w|[, ])+)")
                .unwrap();

        input
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
                    .map(str::to_owned)
                    .collect();
                Valve { name, rate, edges }
            })
            .collect()
    }
}

impl Solution<Day16> for Aoc2022 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Vec<Valve>) -> u32 {
        let interesting_valve_names: Vec<_> = input
            .iter()
            .filter_map(|valve| {
                if valve.rate != 0 || valve.name == "AA" {
                    Some(valve.name.clone())
                } else {
                    None
                }
            })
            .collect();

        let valves: HashMap<_, _> = input
            .iter()
            .map(|valve| (valve.name.clone(), valve.clone()))
            .collect();

        // bfs from all interesting points (and "AA") to other interesting points
        let mut costs = HashMap::default();
        for name in interesting_valve_names {
            costs.insert(
                name.clone(),
                bfs(&valves, name)
                    .into_iter()
                    .filter(|(name, _)| valves[name].rate != 0)
                    .collect::<Vec<_>>(),
            );
        }

        // dfs
        let mut queue = vec![Path {
            nodes: vec!["AA".to_owned()],
            last: "AA".to_owned(),
            total_rate: 0,
            time: 0,
        }];

        let mut res = Vec::new();

        while let Some(current) = queue.pop() {
            let mut found_next = false;
            for (next, cost) in &costs[(&current.last)] {
                if current.nodes.contains(next) {
                    continue;
                }

                let time = current.time + *cost + 1;
                if time > 30 {
                    continue;
                }

                found_next = true;
                let mut nodes = current.nodes.clone();
                nodes.push(next.clone());

                queue.push(Path {
                    nodes,
                    last: next.clone(),
                    total_rate: current.total_rate + valves[next].rate * (30 - time),
                    time,
                })
            }

            if !found_next {
                res.push(current);
            }
        }

        res.into_iter().map(|path| path.total_rate).max().unwrap()
    }

    fn part2(_input: &Vec<Valve>) -> u32 {
        todo!()
    }
}

fn bfs(edges: &HashMap<String, Valve>, start: String) -> HashMap<String, u32> {
    let mut open_queue = VecDeque::new();
    open_queue.push_back((start.clone(), 0));

    let mut visited = HashMap::default();

    while let Some((current, cost)) = open_queue.pop_front() {
        if current != start {
            visited.insert(current.clone(), cost);
        }

        let current = edges.get(&current).unwrap();
        for next_name in &current.edges {
            if !visited.contains_key(next_name) {
                open_queue.push_back((next_name.clone(), cost + 1));
            }
        }
    }
    visited
}

#[derive(Debug)]
struct Path {
    nodes: Vec<String>,
    last: String,
    total_rate: u32,
    time: u32,
}
