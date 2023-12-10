use crate::aoc2022::grid::{Direction, Grid};
use crate::aoc2023::Aoc2023;
use crate::traits::days::Day10;
use crate::traits::ParseInput;
use crate::traits::Solution;

impl ParseInput<Day10> for Aoc2023 {
    type Parsed = Grid<char>;

    fn parse_input(input: &str) -> Self::Parsed {
        Grid::parse(input, |c| c)
    }
}

impl Solution<Day10> for Aoc2023 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Grid<char>) -> u32 {
        let (start_x, start_y, _) = input.iter().find(|(_, _, &val)| val == 'S').unwrap();

        let mut starting_points = Vec::new();
        for (dir, x, y) in input.get_neighbors_with_direction(start_x, start_y) {
            let in_dir = match dir {
                Direction::North => Direction::South,
                Direction::South => Direction::North,
                Direction::East => Direction::West,
                Direction::West => Direction::East,
            };

            if next_cell(input, in_dir, x, y).is_some() {
                starting_points.push((in_dir, x, y));
            }
        }

        let mut farthest = 0;
        'sp: for (mut dir, mut x, mut y) in starting_points {
            let mut steps = 1;
            while *input.get(x, y) != 'S' {
                if let Some((new_dir, (new_x, new_y))) = next_cell(input, dir, x, y) {
                    steps += 1;
                    dir = new_dir;
                    x = new_x;
                    y = new_y;
                } else {
                    continue 'sp;
                }
            }

            if steps > farthest {
                farthest = steps;
            }
        }

        farthest / 2
    }

    fn part2(_input: &Grid<char>) -> u32 {
        todo!()
    }
}

fn next_cell(
    grid: &Grid<char>,
    from: Direction,
    x: usize,
    y: usize,
) -> Option<(Direction, (usize, usize))> {
    let cell = *grid.get(x, y);
    let (dir, (dx, dy)) = match (from, cell) {
        (Direction::North, '|') => (Direction::North, (0, 1)),
        (Direction::South, '|') => (Direction::South, (0, -1)),

        (Direction::East, '-') => (Direction::East, (-1, 0)),
        (Direction::West, '-') => (Direction::West, (1, 0)),

        (Direction::North, 'L') => (Direction::West, (1, 0)),
        (Direction::East, 'L') => (Direction::South, (0, -1)),

        (Direction::West, '7') => (Direction::North, (0, 1)),
        (Direction::South, '7') => (Direction::East, (-1, 0)),

        (Direction::East, 'F') => (Direction::North, (0, 1)),
        (Direction::South, 'F') => (Direction::West, (1, 0)),

        (Direction::North, 'J') => (Direction::East, (-1, 0)),
        (Direction::West, 'J') => (Direction::South, (0, -1)),

        _ => {
            return None;
        }
    };

    if (x == 0 && dx < 0) || (x == grid.width - 1 && dx > 0) {
        return None;
    }

    if (y == 0 && dy < 0) || (y == grid.height - 1 && dy > 0) {
        return None;
    }

    let x = (x as isize + dx) as usize;
    let y = (y as isize + dy) as usize;

    Some((dir, (x, y)))
}
