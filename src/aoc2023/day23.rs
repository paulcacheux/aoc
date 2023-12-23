use std::collections::HashMap;
use std::collections::HashSet;

use crate::aoc2023::Aoc2023;
use crate::grid::Direction;
use crate::grid::Grid;
use crate::traits::days::Day23;
use crate::traits::ParseInput;
use crate::traits::Solution;

impl ParseInput<Day23> for Aoc2023 {
    type Parsed = Grid<char>;

    fn parse_input(input: &str) -> Self::Parsed {
        Grid::parse(input, |c| c)
    }
}

impl Solution<Day23> for Aoc2023 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Grid<char>) -> usize {
        let edges = build_simplified_graph(input);

        let start = (1, 0);
        let end = (input.width - 2, input.height - 1);

        let mut open_queue = vec![(0, start, HashSet::new())];
        let mut longest = 0;

        while let Some((distance, (x, y), mut visited)) = open_queue.pop() {
            if (x, y) == end {
                if distance > longest {
                    longest = distance;
                }
            } else {
                visited.insert((x, y));
            }

            if let Some(edges) = edges.get(&(x, y)) {
                for edge in edges {
                    if !visited.contains(&edge.to) {
                        open_queue.push((distance + edge.distance, edge.to, visited.clone()));
                    }
                }
            }
        }

        longest
    }

    fn part2(_input: &Grid<char>) -> usize {
        todo!()
    }
}

fn build_simplified_graph(input: &Grid<char>) -> HashMap<(usize, usize), HashSet<Edge>> {
    let start = (1, 0);
    let end = (input.width - 2, input.height - 1);

    let mut open_queue = vec![PathPart {
        parent: start,
        pos: start,
        distance: 0,
        visited: HashSet::new(),
    }];
    let mut new_cells = Vec::with_capacity(4);

    let mut graph: HashMap<(usize, usize), HashSet<Edge>> = HashMap::new();

    while let Some(PathPart {
        parent,
        pos: (x, y),
        distance,
        mut visited,
    }) = open_queue.pop()
    {
        visited.insert((x, y));

        let force_direction = match *input.get(x, y) {
            '>' => Some(Direction::East),
            '<' => Some(Direction::West),
            'v' => Some(Direction::South),
            _ => None,
        };

        new_cells.clear();

        for (direction, nx, ny) in input.get_neighbors_with_direction(x, y) {
            if let Some(fdir) = force_direction {
                if fdir != direction {
                    continue;
                }
            }

            match *input.get(nx, ny) {
                '#' => continue,
                '>' if direction != Direction::East => continue,
                '<' if direction != Direction::West => continue,
                'v' if direction != Direction::South => continue,
                _ => {}
            }

            if !visited.contains(&(nx, ny)) {
                new_cells.push((nx, ny));
            }
        }

        if new_cells.len() != 1 || (x, y) == end {
            graph.entry(parent).or_default().insert(Edge {
                to: (x, y),
                distance,
            });
        }

        if new_cells.len() == 1 {
            open_queue.push(PathPart {
                parent,
                pos: new_cells[0],
                distance: distance + 1,
                visited,
            });
        } else {
            for &pos in &new_cells {
                open_queue.push(PathPart {
                    parent: (x, y),
                    pos,
                    distance: 1,
                    visited: visited.clone(),
                });
            }
        }
    }
    graph
}

struct PathPart {
    parent: (usize, usize),
    pos: (usize, usize),
    distance: usize,
    visited: HashSet<(usize, usize)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Edge {
    to: (usize, usize),
    distance: usize,
}
