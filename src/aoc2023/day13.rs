use crate::aoc2022::grid::Grid;
use crate::aoc2023::Aoc2023;
use crate::traits::days::Day13;
use crate::traits::ParseInput;
use crate::traits::Solution;

impl ParseInput<Day13> for Aoc2023 {
    type Parsed = Vec<Grid<bool>>;

    fn parse_input(input: &str) -> Self::Parsed {
        input
            .split("\n\n")
            .map(|group| {
                Grid::parse(group, |c| match c {
                    '#' => true,
                    '.' => false,
                    _ => unreachable!(),
                })
            })
            .collect()
    }
}

impl Solution<Day13> for Aoc2023 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Vec<Grid<bool>>) -> usize {
        let mut score = 0;
        for entry in input {
            // look for x-axis mirror
            'search: for mx in 1..entry.width {
                for x in 0..mx {
                    let opposite_x: usize = mx + (mx - x - 1);
                    if opposite_x >= entry.width {
                        continue;
                    }

                    for y in 0..entry.height {
                        if entry.get(x, y) != entry.get(opposite_x, y) {
                            continue 'search;
                        }
                    }
                }
                score += mx;
            }

            // look for y-axis mirror
            'search: for my in 1..entry.height {
                for y in 0..my {
                    let opposite_y: usize = my + (my - y - 1);
                    if opposite_y >= entry.height {
                        continue;
                    }

                    for x in 0..entry.width {
                        if entry.get(x, y) != entry.get(x, opposite_y) {
                            continue 'search;
                        }
                    }
                }
                score += 100 * my;
            }
        }
        score
    }

    fn part2(input: &Vec<Grid<bool>>) -> usize {
        let mut score = 0;
        for entry in input {
            // look for x-axis mirror
            for mx in 1..entry.width {
                let mut errors = 0;
                for x in 0..mx {
                    let opposite_x: usize = mx + (mx - x - 1);
                    if opposite_x >= entry.width {
                        continue;
                    }

                    for y in 0..entry.height {
                        if entry.get(x, y) != entry.get(opposite_x, y) {
                            errors += 1;
                        }
                    }
                }

                if errors == 1 {
                    score += mx;
                }
            }

            // look for y-axis mirror
            for my in 1..entry.height {
                let mut errors = 0;
                for y in 0..my {
                    let opposite_y: usize = my + (my - y - 1);
                    if opposite_y >= entry.height {
                        continue;
                    }

                    for x in 0..entry.width {
                        if entry.get(x, y) != entry.get(x, opposite_y) {
                            errors += 1;
                        }
                    }
                }

                if errors == 1 {
                    score += 100 * my
                }
            }
        }
        score
    }
}
