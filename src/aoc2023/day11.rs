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
    assert!(empty_columns.is_sorted());

    // look for empty rows
    'row: for y in 0..input.height {
        for x in 0..input.width {
            if *input.get(x, y) {
                continue 'row;
            }
        }

        empty_rows.push(y);
    }
    assert!(empty_rows.is_sorted());

    let galaxies: Vec<_> = input
        .iter()
        .filter_map(|(x, y, &is_galaxy)| {
            if !is_galaxy {
                return None;
            }

            let mut deltax = 0;
            let mut deltay = 0;

            for &col in &empty_columns {
                if col < x {
                    deltax += 1;
                } else {
                    break;
                }
            }

            for &row in &empty_rows {
                if row < y {
                    deltay += 1;
                } else {
                    break;
                }
            }

            let x = x + deltax * (expansion - 1);
            let y = y + deltay * (expansion - 1);

            Some((x, y))
        })
        .collect();

    let mut total = 0;
    for (i, &(ax, ay)) in galaxies.iter().enumerate() {
        for &(bx, by) in &galaxies[(i + 1)..] {
            total += ax.abs_diff(bx) + ay.abs_diff(by);
        }
    }
    total
}
