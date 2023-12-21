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
    type Part2Output = usize;

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

    fn part2(input: &Grid<char>) -> usize {
        let mut start = (0, 0);

        for (x, y, value) in input.iter() {
            if *value == 'S' {
                start = (x as isize, y as isize);
            }
        }

        let mut visited = HashSet::new();
        let mut open_queue = HashSet::new();
        open_queue.insert(start);

        assert_eq!(input.width, input.height);

        let mut counters = (0, 0);
        let mut open_sizes = vec![0; input.width];
        let mut first_derivative = vec![0; input.width];
        let mut second_derivative = vec![0; input.width];

        let mut step = 0;
        loop {
            let mut new_open_queue = HashSet::new();

            for (x, y) in open_queue {
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let (nx, ny) = (x + dx, y + dy);

                    let rx = nx.rem_euclid(input.width as isize) as usize;
                    let ry = ny.rem_euclid(input.height as isize) as usize;

                    if *input.get(rx, ry) != '#' && visited.insert((nx, ny)) {
                        new_open_queue.insert((nx, ny));
                    }
                }
            }

            open_queue = new_open_queue;

            let oqlen = open_queue.len() as isize;

            // update counters
            counters = (counters.1, oqlen + counters.0);

            let stepmod = step % input.width;
            if step >= input.width {
                let delta = oqlen - open_sizes[stepmod];
                second_derivative[stepmod] = delta - first_derivative[stepmod];
                first_derivative[stepmod] = delta;
            }
            open_sizes[stepmod] = oqlen;

            step += 1;

            if step >= input.width * 2 && second_derivative.iter().all(|&d| d == 0) {
                break;
            }
        }

        for i in step..26501365 {
            let imod = i % input.width;
            open_sizes[imod] += first_derivative[imod];
            counters = (counters.1, open_sizes[imod] + counters.0);
        }

        counters.1 as usize
    }
}
