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
        find_longest_path(input, true)
    }

    fn part2(input: &Grid<char>) -> usize {
        find_longest_path(input, false)
    }
}

fn find_longest_path(input: &Grid<char>, with_slopes: bool) -> usize {
    let edges = build_simplified_graph(input, with_slopes);

    let start = (1, 0);
    let end = (input.width - 2, input.height - 1);

    let mut open_queue = vec![(0, start, Vec::new())];
    let mut longest = 0;

    while let Some((distance, (x, y), mut visited)) = open_queue.pop() {
        if (x, y) == end {
            if distance > longest {
                longest = distance;
            }
            continue;
        } else {
            visited.push((x, y));
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

fn build_simplified_graph(
    input: &Grid<char>,
    with_slopes: bool,
) -> HashMap<(usize, usize), HashSet<Edge>> {
    let start = (1, 0);
    let end = (input.width - 2, input.height - 1);

    let mut open_queue = vec![PathPart {
        parent: start,
        pos: start,
        distance: 0,
    }];
    let mut new_cells = Vec::with_capacity(4);

    let mut graph: HashMap<(usize, usize), HashSet<Edge>> = HashMap::new();
    let mut visited = HashSet::new();

    while let Some(PathPart {
        parent,
        pos: (x, y),
        distance,
        ..
    }) = open_queue.pop()
    {
        new_cells.clear();

        let force_direction = if with_slopes {
            match *input.get(x, y) {
                '>' => Some(Direction::East),
                '<' => Some(Direction::West),
                'v' => Some(Direction::South),
                '^' => Some(Direction::North),
                _ => None,
            }
        } else {
            None
        };

        let mut neighbors_count = 0;
        for (direction, nx, ny) in input.get_neighbors_with_direction(x, y) {
            let cell = *input.get(nx, ny);

            if cell == '#' {
                continue;
            }
            neighbors_count += 1;

            if with_slopes {
                if let Some(fdir) = force_direction {
                    if fdir != direction {
                        continue;
                    }
                }

                match *input.get(nx, ny) {
                    '>' if direction != Direction::East => continue,
                    '<' if direction != Direction::West => continue,
                    'v' if direction != Direction::South => continue,
                    '^' if direction != Direction::North => continue,
                    _ => {}
                }
            } else if *input.get(nx, ny) == '#' {
                continue;
            }

            if !visited.contains(&(nx, ny, parent)) {
                new_cells.push((nx, ny));
            }
        }

        if neighbors_count <= 2 && (x, y) != end {
            visited.insert((x, y, parent));
        }

        if (neighbors_count > 2 || (x, y) == end) && parent != (x, y) {
            graph.entry(parent).or_default().insert(Edge {
                to: (x, y),
                distance,
            });

            if !with_slopes {
                graph.entry((x, y)).or_default().insert(Edge {
                    to: parent,
                    distance,
                });
            }
        }

        let (parent, base_distance) = if neighbors_count <= 2 {
            (parent, distance)
        } else {
            ((x, y), 0)
        };

        for &pos in &new_cells {
            open_queue.push(PathPart {
                parent,
                pos,
                distance: base_distance + 1,
            });
        }
    }
    graph
}

struct PathPart {
    parent: (usize, usize),
    pos: (usize, usize),
    distance: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Edge {
    to: (usize, usize),
    distance: usize,
}
