use std::ops::RangeInclusive;

use ahash::HashSet;
use ahash::HashSetExt;
use itertools::Itertools;
use itertools::MinMaxResult;

use super::grid::Grid;
use crate::aoc2022::Aoc2022;
use crate::traits::days::Day23;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Elf,
}

impl ParseInput<Day23> for Aoc2022 {
    type Parsed = Grid<Cell>;

    fn parse_input(input: &str) -> Self::Parsed {
        Grid::parse(input, |c| match c {
            '#' => Cell::Elf,
            '.' => Cell::Empty,
            _ => unreachable!(),
        })
    }
}

impl Solution<Day23> for Aoc2022 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Grid<Cell>) -> usize {
        // build first state
        let mut state = first_state(input);

        // loop over it
        for start_di in 0..10 {
            state = next_state(&state, start_di);
        }

        score(&state)
    }

    fn part2(input: &Grid<Cell>) -> usize {
        // build first state
        let mut state = first_state(input);

        // loop over it
        let mut start_di = 0;
        loop {
            let next_state = next_state(&state, start_di);
            start_di += 1;
            if state == next_state {
                return start_di;
            }
            state = next_state;
        }
    }
}

const DELTAS: [[(isize, isize); 3]; 4] = [
    [(-1, -1), (0, -1), (1, -1)], // N
    [(-1, 1), (0, 1), (1, 1)],    // S
    [(-1, -1), (-1, 0), (-1, 1)], // W
    [(1, -1), (1, 0), (1, 1)],    // E
];

fn next_state(state: &HashSet<(isize, isize)>, start_di: usize) -> HashSet<(isize, isize)> {
    let mut next_state = HashSet::with_capacity(state.len());
    let mut status = HashSet::with_capacity(state.len());
    for &(x, y) in state {
        let mut all_suitable = true;
        let mut final_di = None;
        for di in 0..DELTAS.len() {
            let di = (start_di + di) % DELTAS.len();

            let suitable = DELTAS[di].iter().all(|&(dx, dy)| {
                let next = (x + dx, y + dy);
                !state.contains(&next)
            });

            if suitable {
                if final_di.is_none() {
                    final_di = Some(di);
                }
            } else {
                all_suitable = false;
            }
        }

        if all_suitable {
            next_state.insert((x, y));
            continue;
        }

        if let Some(di) = final_di {
            let (dx, dy) = DELTAS[di][1];
            let next = (x + dx, y + dy);

            if status.contains(&next) {
                // this is possible because at most 2 elfs fight for the same spot,
                // and if they do they always come from opposite directions
                let (ox, oy) = (x + 2 * dx, y + 2 * dy);
                assert!(next_state.get(&next).is_some());
                next_state.remove(&next);
                next_state.insert((ox, oy));
                next_state.insert((x, y));
            } else {
                assert!(next_state.get(&next).is_none());
                next_state.insert(next);
                status.insert(next);
            }
        } else {
            next_state.insert((x, y));
        }
    }
    assert_eq!(next_state.len(), state.len());
    normalize(next_state)
}

fn normalize(state: HashSet<(isize, isize)>) -> HashSet<(isize, isize)> {
    let mut xmin = isize::MAX;
    let mut ymin = isize::MAX;
    for &(x, y) in state.iter() {
        if x < xmin {
            xmin = x;
        }
        if y < ymin {
            ymin = y;
        }
    }

    state
        .into_iter()
        .map(|(x, y)| (x - xmin, y - ymin))
        .collect()
}

fn score(grid: &HashSet<(isize, isize)>) -> usize {
    fn len(mm: MinMaxResult<isize>) -> isize {
        match mm {
            itertools::MinMaxResult::NoElements => unreachable!(),
            itertools::MinMaxResult::OneElement(_) => 1,
            itertools::MinMaxResult::MinMax(a, b) => b - a + 1,
        }
    }

    let xminmax = grid.iter().map(|p| p.0).minmax();
    let width = len(xminmax) as usize;
    let yminmax = grid.iter().map(|p| p.1).minmax();
    let height = len(yminmax) as usize;

    width * height - grid.len()
}

#[allow(dead_code)]
fn dbg_grid(grid: &HashSet<(isize, isize)>) {
    fn range(mm: MinMaxResult<isize>) -> RangeInclusive<isize> {
        match mm {
            itertools::MinMaxResult::NoElements => unreachable!(),
            itertools::MinMaxResult::OneElement(x) => x..=x,
            itertools::MinMaxResult::MinMax(a, b) => a..=b,
        }
    }

    let xminmax = grid.iter().map(|p| p.0).minmax();
    let xrange = range(xminmax);
    let yminmax = grid.iter().map(|p| p.1).minmax();
    let yrange = range(yminmax);

    for y in yrange {
        for x in xrange.clone() {
            if grid.contains(&(x, y)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }
}

fn first_state(grid: &Grid<Cell>) -> HashSet<(isize, isize)> {
    grid.iter()
        .filter_map(|(x, y, val)| {
            if *val == Cell::Elf {
                Some((x as isize, y as isize))
            } else {
                None
            }
        })
        .collect()
}
