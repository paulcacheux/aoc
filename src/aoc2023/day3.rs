use std::collections::HashMap;

use crate::aoc2022::grid::Grid;
use crate::aoc2023::Aoc2023;
use crate::traits::days::Day3;
use crate::traits::ParseInput;
use crate::traits::Solution;

impl ParseInput<Day3> for Aoc2023 {
    type Parsed = Grid<u8>;

    fn parse_input(input: &str) -> Self::Parsed {
        Grid::parse(input, |c| c as u8)
    }
}

#[derive(Debug, Clone, Copy)]
struct Entry {
    x: usize,
    y: usize,
    len: usize,
    value: u32,
}

impl Entry {
    fn iter_neighbors(&self, grid: &Grid<u8>) -> Vec<(usize, usize)> {
        let x = self.x as i32;
        let y = self.y as i32;
        let len = self.len as i32;
        let mut res = Vec::with_capacity(6 + 2 * self.len);
        res.extend_from_slice(&[(x - 1, y - 1), (x - 1, y), (x - 1, y + 1)]);
        for dx in 0..len {
            res.extend_from_slice(&[(x + dx, y - 1), (x + dx, y + 1)]);
        }
        res.extend_from_slice(&[(x + len, y - 1), (x + len, y), (x + len, y + 1)]);

        res.into_iter()
            .filter_map(|(x, y)| {
                if x >= 0 && y >= 0 && x < grid.width as i32 && y < grid.height as i32 {
                    Some((x as _, y as _))
                } else {
                    None
                }
            })
            .collect()
    }
}

fn get_entries(input: &Grid<u8>) -> impl Iterator<Item = Entry> + '_ {
    std::iter::from_generator(move || {
        let mut current = None;
        for y in 0..input.height {
            for x in 0..input.width {
                let c = *input.get(x, y);
                if c.is_ascii_digit() {
                    let digit = (c - b'0') as u32;

                    current = match current {
                        Some(Entry { x, y, len, value }) => Some(Entry {
                            x,
                            y,
                            len: len + 1,
                            value: value * 10 + digit,
                        }),
                        None => Some(Entry {
                            x,
                            y,
                            len: 1,
                            value: digit,
                        }),
                    }
                } else if let Some(entry) = current.take() {
                    yield entry;
                }
            }
        }

        if let Some(entry) = current {
            yield entry;
        }
    })
}

impl Solution<Day3> for Aoc2023 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Grid<u8>) -> u32 {
        let mut res = 0;
        for entry in get_entries(input) {
            for (x, y) in entry.iter_neighbors(input) {
                if *input.get(x, y) != b'.' {
                    res += entry.value;
                    break;
                }
            }
        }
        res
    }

    fn part2(input: &Grid<u8>) -> u32 {
        let mut gears: HashMap<(usize, usize), Vec<Entry>> = HashMap::new();

        for entry in get_entries(input) {
            for (x, y) in entry.iter_neighbors(input) {
                if *input.get(x, y) == b'*' {
                    gears.entry((x, y)).or_default().push(entry);
                }
            }
        }

        gears
            .values()
            .filter(|entries| entries.len() == 2)
            .map(|entries| entries.iter().map(|entry| entry.value).product::<u32>())
            .sum()
    }
}
