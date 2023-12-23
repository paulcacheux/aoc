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
    let start = (1, 0);
    let end = (input.width - 2, input.height - 1);
    assert_eq!(*input.get(start.0, start.1), '.');
    assert_eq!(*input.get(end.0, end.1), '.');

    let mut open_queue = vec![(0, start, HashSet::new())];
    let mut longest = 0;
    let mut new_cells = Vec::with_capacity(4);

    while let Some((distance, (x, y), mut visited)) = open_queue.pop() {
        if (x, y) == end {
            if distance > longest {
                longest = distance;
            }
        } else {
            visited.insert((x, y));
        }

        let force_direction = if with_slopes {
            match *input.get(x, y) {
                '>' => Some(Direction::East),
                '<' => Some(Direction::West),
                'v' => Some(Direction::South),
                _ => None,
            }
        } else {
            None
        };

        new_cells.clear();

        for (direction, nx, ny) in input.get_neighbors_with_direction(x, y) {
            if with_slopes {
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
            } else if *input.get(nx, ny) == '#' {
                continue;
            }

            if !visited.contains(&(nx, ny)) {
                new_cells.push((nx, ny));
            }
        }

        if new_cells.len() == 1 {
                open_queue.push((distance + 1, new_cells[0], visited));
        } else {
            for &cell in &new_cells {
                open_queue.push((distance + 1, cell, visited.clone()));
            }
        }
    }
    longest
}
