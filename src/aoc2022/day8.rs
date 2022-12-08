use crate::aoc2022::Aoc2022;
use crate::traits::days::Day8;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug)]
pub struct Grid {
    heights: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> u8 {
        self.heights[self.width * y + x]
    }
}

impl ParseInput<Day8> for Aoc2022 {
    type Parsed = Grid;

    fn parse_input(input: &str) -> Self::Parsed {
        let mut heights = Vec::new();
        let mut width = None;
        let mut height = 0;

        for line in input.lines() {
            let line = line.trim();
            if let Some(w) = width {
                assert_eq!(w, line.len());
            } else {
                width = Some(line.len());
            }
            heights.extend(line.chars().map(|c| c.to_string().parse::<u8>().unwrap()));
            height += 1;
        }

        Grid {
            heights,
            width: width.unwrap_or_default(),
            height,
        }
    }
}

impl Solution<Day8> for Aoc2022 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Grid) -> u32 {
        let mut side = 0;
        let mut counter = 0;
        for y in 0..input.height {
            for x in 0..input.width {
                let current_tree = input.get(x, y);

                if x == 0 || y == 0 || x == input.width - 1 || y == input.height - 1 {
                    side += 1;
                    continue;
                }

                // left
                if (0..x).all(|dx| input.get(dx, y) < current_tree) {
                    counter += 1;
                    continue;
                }
                // right
                if ((x + 1)..input.width).all(|dx| input.get(dx, y) < current_tree) {
                    counter += 1;
                    continue;
                }
                // up
                if (0..y).all(|dy| input.get(x, dy) < current_tree) {
                    counter += 1;
                    continue;
                }
                // down
                if ((y + 1)..input.height).all(|dy| input.get(x, dy) < current_tree) {
                    counter += 1;
                    continue;
                }
            }
        }
        side + counter
    }

    fn part2(input: &Grid) -> u32 {
        todo!()
    }
}
