use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graphmap::UnGraphMap;

use crate::aoc2023::Aoc2023;
use crate::traits::days::Day25;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug, Clone)]
pub struct Edges {
    from: String,
    to: Vec<String>,
}

impl ParseInput<Day25> for Aoc2023 {
    type Parsed = Vec<Edges>;

    fn parse_input(input: &str) -> Self::Parsed {
        input
            .lines()
            .map(|line| {
                let (from, to) = line.split_once(": ").unwrap();
                Edges {
                    from: from.to_owned(),
                    to: to.split_ascii_whitespace().map(str::to_owned).collect(),
                }
            })
            .collect()
    }
}

impl Solution<Day25> for Aoc2023 {
    type Part1Output = usize;
    type Part2Output = u32;

    fn part1(input: &Vec<Edges>) -> usize {
        let mut graph = UnGraphMap::new();
        for edge in input {
            for to in &edge.to {
                graph.add_edge(edge.from.as_str(), to.as_str(), ());
            }
        }

        let (_, group) = stoer_wagner_min_cut(&graph, |_| Ok::<usize, ()>(1))
            .unwrap()
            .unwrap();

        group.len() * (graph.node_count() - group.len())
    }

    fn part2(_input: &Vec<Edges>) -> u32 {
        todo!()
    }
}
