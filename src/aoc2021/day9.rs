use crate::aoc2021::Aoc2021;
use crate::traits::days::Day9;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug)]
pub struct PuzzleInput {
    width: usize,
    height: usize,
    values: Vec<u32>,
}

impl PuzzleInput {
    fn get(&self, x: usize, y: usize) -> u32 {
        self.values[y * self.width + x]
    }

    fn get_neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let x = x as i32;
        let y = y as i32;
        let width = self.width;
        let height = self.height;

        let deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        deltas.into_iter().flat_map(move |(dx, dy)| {
            let rx = x + dx;
            let ry = y + dy;

            if 0 <= rx && rx < width as _ && 0 <= ry && ry < height as _ {
                Some((rx as usize, ry as usize))
            } else {
                None
            }
        })
    }
}

impl ParseInput<Day9> for Aoc2021 {
    type Parsed = PuzzleInput;

    fn parse_input(input: &str) -> PuzzleInput {
        let mut values = Vec::new();
        let mut width = None;
        let mut height = 0;
        for line in input.lines() {
            height += 1;

            let line = line.trim();
            if let Some(width) = width {
                assert_eq!(width, line.len());
            } else {
                width = Some(line.len());
            }

            for c in line.chars() {
                values.push(c.to_digit(10).unwrap())
            }
        }

        PuzzleInput {
            width: width.unwrap(),
            height,
            values,
        }
    }
}

#[derive(Debug)]
struct ColoredMap {
    width: usize,
    colors: Vec<u32>,
    color_counter: u32,
}

impl ColoredMap {
    fn get(&mut self, x: usize, y: usize) -> u32 {
        self.colors[y * self.width + x]
    }

    fn set(&mut self, x: usize, y: usize, color: u32) {
        self.colors[y * self.width + x] = color;
    }

    fn next_color(&mut self) -> u32 {
        self.color_counter += 1;
        self.color_counter
    }
}

fn compute_lower_points(map: &PuzzleInput) -> Vec<(usize, usize)> {
    let mut points = Vec::new();
    for y in 0..map.height {
        'main: for x in 0..map.width {
            let current = map.get(x, y);

            for (rx, ry) in map.get_neighbors(x, y) {
                if map.get(rx as usize, ry as usize) <= current {
                    continue 'main;
                }
            }
            points.push((x, y))
        }
    }
    points
}

impl Solution<Day9> for Aoc2021 {
    type Part1Output = u32;
    type Part2Output = usize;

    fn part1(input: &PuzzleInput) -> u32 {
        compute_lower_points(input)
            .into_iter()
            .map(|(x, y)| input.get(x, y) + 1)
            .sum()
    }

    fn part2(input: &PuzzleInput) -> usize {
        let mut colored_map = ColoredMap {
            width: input.width,
            colors: vec![0; input.values.len()],
            color_counter: 0,
        };

        for (x, y) in compute_lower_points(input) {
            let color = colored_map.next_color();

            let mut open_list = vec![(x, y)];

            while let Some((x, y)) = open_list.pop() {
                if colored_map.get(x, y) != 0 {
                    continue;
                }
                if input.get(x, y) == 9 {
                    continue;
                }

                colored_map.set(x, y, color);
                open_list.extend(input.get_neighbors(x, y));
            }
        }

        let mut color_count = vec![0; colored_map.color_counter as usize + 1];
        for color in colored_map.colors {
            if color != 0 {
                color_count[color as usize] += 1;
            }
        }

        color_count.sort_unstable();
        color_count.into_iter().rev().take(3).product()
    }
}
