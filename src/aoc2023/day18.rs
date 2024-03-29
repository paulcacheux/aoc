use crate::aoc2023::Aoc2023;
use crate::grid::Direction;
use crate::traits::days::Day18;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug)]
pub struct InputLine {
    dir: Direction,
    count: usize,
    color: String,
}

impl ParseInput<Day18> for Aoc2023 {
    type Parsed = Vec<InputLine>;

    fn parse_input(input: &str) -> Self::Parsed {
        input
            .lines()
            .map(|line| {
                let mut words = line.split_ascii_whitespace();
                let dir = match words.next().unwrap() {
                    "R" => Direction::East,
                    "L" => Direction::West,
                    "U" => Direction::North,
                    "D" => Direction::South,
                    _ => unreachable!(),
                };

                let count = words.next().unwrap().parse().unwrap();

                let color = words.next().unwrap();
                let color = color[2..color.len() - 1].to_owned();

                InputLine { dir, count, color }
            })
            .collect()
    }
}

impl Solution<Day18> for Aoc2023 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Vec<InputLine>) -> usize {
        solve(input.iter().map(|line| (line.dir, line.count)))
    }

    fn part2(input: &Vec<InputLine>) -> usize {
        solve(input.iter().map(|line| {
            let count = usize::from_str_radix(&line.color[..5], 16).unwrap();
            let dir = match &line.color[5..] {
                "0" => Direction::East,
                "1" => Direction::South,
                "2" => Direction::West,
                "3" => Direction::North,
                _ => unreachable!(),
            };
            (dir, count)
        }))
    }
}

fn solve<I: Iterator<Item = (Direction, usize)>>(instructions: I) -> usize {
    let (mut x, mut y) = (0isize, 0isize);

    let mut area = 0;

    for (dir, count) in instructions {
        let (dx, dy) = match dir {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
        };

        let count = count as isize;
        let (dx, dy) = (dx * count, dy * count);
        let (nx, ny) = (x + dx, y + dy);

        area += dx.abs() + dy.abs() + x * ny - nx * y;

        (x, y) = (nx, ny);
    }

    let area = area / 2 + 1;
    area.unsigned_abs()
}
