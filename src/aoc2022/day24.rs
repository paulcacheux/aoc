use std::collections::HashSet;

use crate::aoc2022::Aoc2022;
use crate::traits::days::Day24;
use crate::traits::ParseInput;
use crate::traits::Solution;

use super::grid::Grid;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
#[repr(u8)]
pub enum Cell {
    Left,
    Right,
    Up,
    Down,
    Empty,
    Wall,
}

impl ParseInput<Day24> for Aoc2022 {
    type Parsed = Grid<Cell>;

    fn parse_input(input: &str) -> Self::Parsed {
        Grid::parse(input, |c| match c {
            '.' => Cell::Empty,
            '#' => Cell::Wall,
            '<' => Cell::Left,
            '>' => Cell::Right,
            '^' => Cell::Up,
            'v' => Cell::Down,
            _ => unreachable!(),
        })
    }
}

type Coords = (isize, isize);

impl Solution<Day24> for Aoc2022 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Grid<Cell>) -> u32 {
        solve::<1>(input)
    }

    fn part2(input: &Grid<Cell>) -> u32 {
        solve::<2>(input)
    }
}

fn solve<const PART: usize>(input: &Grid<Cell>) -> u32 {
    const DELTAS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let width = input.width - 2;
    let height = input.height - 2;

    let wrap =
        |(x, y): (isize, isize)| (x.rem_euclid(width as isize), y.rem_euclid(height as isize));

    let mut bliz: Vec<(Cell, Coords)> = Default::default();
    for dir in [Cell::Left, Cell::Right, Cell::Up, Cell::Down] {
        for y in 0..height {
            for x in 0..width {
                if *input.get(x + 1, y + 1) == dir {
                    bliz.push((dir, (x as isize, y as isize)));
                }
            }
        }
    }

    let home = (0, -1);
    let goal = (width as isize - 1, height as isize);

    let mut open_queue = vec![home];
    let mut time = 0;
    let mut trip = 0;

    while !open_queue.is_empty() {
        for (dir, pt) in bliz.iter_mut() {
            let (dx, dy) = DELTAS[*dir as usize];
            *pt = wrap((pt.0 + dx, pt.1 + dy));
        }

        time += 1;
        let mut curr = HashSet::with_capacity(open_queue.len() * 5);
        for (px, py) in open_queue.drain(..) {
            curr.extend(
                [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)].map(|(dx, dy)| (px + dx, py + dy)),
            );
        }

        for p in curr {
            match PART {
                1 => {
                    if p == goal {
                        return time;
                    }
                }
                2 => {
                    if trip == 2 && p == goal {
                        return time;
                    }

                    if (trip == 0 && p == goal) || (trip == 1 && p == home) {
                        trip += 1;
                        open_queue.clear();
                        open_queue.push(p);
                        break;
                    }
                }
                _ => unreachable!(),
            }

            if wrap(p) == p && !bliz.iter().map(|(_, pts)| pts).any(|b| *b == p)
                || home == p
                || goal == p
            {
                open_queue.push(p);
            }
        }
    }
    unreachable!()
}
