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

    fn iter<'a>(&'a self) -> impl Iterator<Item = (usize, usize, u8)> + 'a {
        self.heights
            .iter()
            .enumerate()
            .map(|(i, &val)| (i % self.width, i / self.width, val))
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
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Grid) -> usize {
        input
            .iter()
            .filter(|&(x, y, val)| {
                (0..x).all(|dx| input.get(dx, y) < val) // left
                    || ((x + 1)..input.width).all(|dx| input.get(dx, y) < val) // right
                    || (0..y).all(|dy| input.get(x, dy) < val) // up
                    || ((y + 1)..input.height).all(|dy| input.get(x, dy) < val) // down
            })
            .count()
    }

    fn part2(input: &Grid) -> usize {
        input
            .iter()
            .filter_map(|(x, y, current_tree)| {
                if x == 0 || y == 0 || x == input.width - 1 || y == input.height - 1 {
                    return None;
                }

                let mut score = 1;
                // left
                score *= part2_search(0..x, true, |dx| input.get(dx, y) < current_tree);
                // right
                score *= part2_search((x + 1)..input.width, false, |dx| {
                    input.get(dx, y) < current_tree
                });

                // up
                score *= part2_search(0..y, true, |dy| input.get(x, dy) < current_tree);
                // down
                score *= part2_search((y + 1)..input.height, false, |dy| {
                    input.get(x, dy) < current_tree
                });

                Some(score)
            })
            .max()
            .unwrap()
    }
}

fn part2_search<F>(range: std::ops::Range<usize>, rev: bool, check: F) -> usize
where
    F: Fn(usize) -> bool,
{
    let range_len = range.len();
    let count = if rev {
        range.rev().take_while(|&dx| check(dx)).count()
    } else {
        range.take_while(|&dx| check(dx)).count()
    };

    if count != range_len {
        count + 1
    } else {
        count
    }
}
