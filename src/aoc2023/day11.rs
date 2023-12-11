use crate::aoc2022::grid::Grid;
use crate::aoc2023::Aoc2023;
use crate::traits::days::Day11;
use crate::traits::ParseInput;
use crate::traits::Solution;

impl ParseInput<Day11> for Aoc2023 {
    type Parsed = Grid<bool>;

    fn parse_input(input: &str) -> Self::Parsed {
        Grid::parse(input, |c| match c {
            '.' => false,
            '#' => true,
            _ => unreachable!(),
        })
    }
}

impl Solution<Day11> for Aoc2023 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Grid<bool>) -> usize {
        solve(input, 2)
    }

    fn part2(input: &Grid<bool>) -> usize {
        solve(input, 1000000)
    }
}

fn solve(input: &Grid<bool>, expansion: usize) -> usize {
    let mut empty_columns = Vec::new();
    let mut empty_rows = Vec::new();

    // look for empty columns
    'col: for x in 0..input.width {
        for y in 0..input.height {
            if *input.get(x, y) {
                continue 'col;
            }
        }

        empty_columns.push(x);
    }

    // look for empty rows
    'row: for y in 0..input.height {
        for x in 0..input.width {
            if *input.get(x, y) {
                continue 'row;
            }
        }

        empty_rows.push(y);
    }

    let galaxies: Vec<_> = input
        .iter()
        .filter_map(
            |(x, y, &is_galaxy)| {
                if is_galaxy {
                    Some((x, y))
                } else {
                    None
                }
            },
        )
        .collect();

    let mut total = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let (ax, ay) = galaxies[i];
            let (bx, by) = galaxies[j];

            let mut dx = ax.abs_diff(bx);
            for &col in &empty_columns {
                if (ax < col && col < bx) || (bx < col && col < ax) {
                    dx += expansion - 1;
                }
            }

            let mut dy = ay.abs_diff(by);
            for &row in &empty_rows {
                if (ay < row && row < by) || (by < row && row < ay) {
                    dy += expansion - 1;
                }
            }

            let distance = dx + dy;
            total += distance;
        }
    }
    total
}
