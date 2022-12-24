use ahash::HashSet;

use crate::aoc2022::Aoc2022;
use crate::traits::days::Day24;
use crate::traits::ParseInput;
use crate::traits::Solution;

use super::grid::Grid;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Cell {
    Empty,
    Wall,
    Left,
    Right,
    Up,
    Down,
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
        let width = input.width - 2;
        let height = input.height - 2;

        let wrap =
            |(x, y): (isize, isize)| (x.rem_euclid(width as isize), y.rem_euclid(height as isize));

        let mut bliz: Vec<(Cell, HashSet<Coords>)> = Default::default();
        for dir in [Cell::Left, Cell::Right, Cell::Up, Cell::Down] {
            let mut points = HashSet::default();
            for y in 0..height {
                for x in 0..width {
                    if *input.get(x + 1, y + 1) == dir {
                        points.insert((x as isize, y as isize));
                    }
                }
            }
            bliz.push((dir, points));
        }

        let home = (0, -1);
        let goal = (width as isize - 1, height as isize);

        let mut open_queue = vec![home];
        let mut time = 0;

        while !open_queue.is_empty() {
            for (dir, points) in bliz.iter_mut() {
                let (dx, dy) = match dir {
                    Cell::Left => (-1, 0),
                    Cell::Right => (1, 0),
                    Cell::Up => (0, -1),
                    Cell::Down => (0, 1),
                    _ => unreachable!(),
                };

                *points = points
                    .iter()
                    .copied()
                    .map(|(x, y)| wrap((x + dx, y + dy)))
                    .collect();
            }

            time += 1;
            let mut curr = HashSet::default();
            for (px, py) in open_queue.drain(..) {
                for (dx, dy) in [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)] {
                    curr.insert((px + dx, py + dy));
                }
            }

            for p in curr {
                if p == goal {
                    return time;
                }

                if !bliz.iter().map(|(_, pts)| pts).flatten().any(|b| *b == p) && wrap(p) == p
                    || [home, goal].contains(&p)
                {
                    open_queue.push(p);
                }
            }
        }
        todo!()
    }

    fn part2(_input: &Grid<Cell>) -> u32 {
        todo!()
    }
}
