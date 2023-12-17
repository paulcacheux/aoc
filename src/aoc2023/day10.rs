use crate::aoc2023::Aoc2023;
use crate::grid::{Direction, Grid};
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
        let (_, distance) = find_longest(input);
        distance
    }

    fn part2(input: &Grid<char>) -> u32 {
        let (colored_grid, _) = find_longest(input);

        let mut first_value = (0, 0);
        let mut last_value = (0, 0);
        let mut last_value_index = 0;
        for (x, y, value) in colored_grid.iter() {
            if let Some(value) = value {
                if *value == 1 {
                    first_value = (x as i32, y as i32);
                }
                if *value > last_value_index {
                    last_value = (x as i32, y as i32);
                    last_value_index = *value;
                }
            }
        }

        let delta = (last_value.0 - first_value.0, last_value.1 - first_value.1);

        let svalue = match delta {
            (1, -1) => 'F',
            _ => todo!(), // could implement those, but not used in test or actual input.. so let's keep it like that for now
        };

        let mut counter = 0;
        let mut sign = false;
        let mut wall_stack = Vec::new();

        for y in 0..colored_grid.height {
            for x in 0..colored_grid.width {
                if colored_grid.get(x, y).is_some() {
                    let mut cell_value = *input.get(x, y);
                    if cell_value == 'S' {
                        cell_value = svalue;
                    }

                    match cell_value {
                        '|' => sign = !sign,
                        wall @ ('L' | 'F') => {
                            wall_stack.push(wall);
                        }
                        unwall @ ('J' | '7') => {
                            let wall = wall_stack.pop().unwrap();
                            match (wall, unwall) {
                                ('L', '7') | ('F', 'J') => sign = !sign,
                                ('L', 'J') | ('F', '7') => {}
                                _ => unreachable!(),
                            }
                        }
                        _ => {}
                    }
                } else if sign {
                    counter += 1;
                }
            }
            wall_stack.clear();
        }
        counter
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

fn find_longest(input: &Grid<char>) -> (Grid<Option<u32>>, u32) {
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

    starting_points
        .into_iter()
        .filter_map(|(mut dir, mut x, mut y)| {
            let mut colored_grid = Grid::new(input.width, input.height, None);
            colored_grid.set(start_x, start_y, Some(0));

            let mut steps = 1;
            while *input.get(x, y) != 'S' {
                colored_grid.set(x, y, Some(steps));
                if let Some((new_dir, (new_x, new_y))) = next_cell(input, dir, x, y) {
                    steps += 1;
                    dir = new_dir;
                    x = new_x;
                    y = new_y;
                } else {
                    return None;
                }
            }
            Some((colored_grid, steps / 2))
        })
        .max_by_key(|(_, distance)| *distance)
        .unwrap()
}
