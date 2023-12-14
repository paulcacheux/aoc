use crate::aoc2022::grid::Grid;
use crate::aoc2023::Aoc2023;
use crate::traits::days::Day14;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cell {
    Cube,
    Round,
    Empty,
}

impl ParseInput<Day14> for Aoc2023 {
    type Parsed = Grid<Cell>;

    fn parse_input(input: &str) -> Self::Parsed {
        Grid::parse(input, |c| match c {
            '#' => Cell::Cube,
            'O' => Cell::Round,
            '.' => Cell::Empty,
            _ => unreachable!(),
        })
    }
}

impl Solution<Day14> for Aoc2023 {
    type Part1Output = usize;
    type Part2Output = u32;

    fn part1(input: &Grid<Cell>) -> usize {
        let mut grid = input.clone();
        let mut load = 0;

        for x in 0..grid.width {
            for y in 0..grid.height {
                if *grid.get(x, y) == Cell::Round {
                    let mut ny = y;
                    while ny > 0 {
                        if *grid.get(x, ny - 1) == Cell::Empty {
                            ny -= 1;
                        } else {
                            break;
                        }
                    }

                    if ny != y {
                        grid.set(x, y, Cell::Empty);
                        grid.set(x, ny, Cell::Round);
                    }
                    load += grid.height - ny;
                }
            }
        }

        /*
        for y in 0..grid.height {
            for x in 0..grid.width {
                match grid.get(x, y) {
                    Cell::Empty => print!("."),
                    Cell::Round => print!("O"),
                    Cell::Cube => print!("#"),
                }
            }
            println!();
        }
        */

        load
    }

    fn part2(_input: &Grid<Cell>) -> u32 {
        todo!()
    }
}
