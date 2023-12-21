use std::collections::HashSet;

use crate::aoc2023::Aoc2023;
use crate::grid::Grid;
use crate::traits::days::Day21;
use crate::traits::ParseInput;
use crate::traits::Solution;

impl ParseInput<Day21> for Aoc2023 {
    type Parsed = Grid<char>;

    fn parse_input(input: &str) -> Self::Parsed {
        Grid::parse(input, |c| c)
    }
}

impl Solution<Day21> for Aoc2023 {
    type Part1Output = usize;
    type Part2Output = u32;

    fn part1(input: &Grid<char>) -> usize {
        let mut start = (0, 0);

        for (x, y, value) in input.iter() {
            if *value == 'S' {
                start = (x, y);
            }
        }

        let mut open_set = HashSet::new();
        open_set.insert(start);

        for _ in 0..64 {
            let mut new_open_set = HashSet::new();

            for (x, y) in open_set {
                for (nx, ny) in input.get_neighbors(x, y) {
                    if *input.get(nx, ny) != '#' {
                        new_open_set.insert((nx, ny));
                    }
                }
            }

            open_set = new_open_set;
        }

        open_set.len()
    }

    fn part2(_input: &Grid<char>) -> u32 {
        todo!()
    }
}
