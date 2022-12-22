use crate::aoc2022::Aoc2022;
use crate::traits::days::Day22;
use crate::traits::ParseInput;
use crate::traits::Solution;

use super::grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Wall,
}

#[derive(Debug)]
pub enum Instruction {
    Move(u8),
    Left,
    Right,
}

#[derive(Debug)]
pub struct Input {
    grid: Grid<Option<Cell>>,
    instructions: Vec<Instruction>,
}

impl ParseInput<Day22> for Aoc2022 {
    type Parsed = Input;

    fn parse_input(input: &str) -> Self::Parsed {
        let mut predata = Vec::new();
        let mut width = 0;
        let mut height = 0;

        let mut line_iter = input.lines();

        for line in &mut line_iter {
            let line = line.trim_end();

            if line.is_empty() {
                break;
            }

            width = std::cmp::max(width, line.len());

            predata.push(
                line.chars()
                    .map(|c| match c {
                        ' ' => None,
                        '.' => Some(Cell::Empty),
                        '#' => Some(Cell::Wall),
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>(),
            );
            height += 1;
        }

        let mut data = Vec::with_capacity(predata.len() * width);

        for line in predata {
            let missing = width - line.len();
            data.extend(line);
            data.extend(std::iter::repeat(None).take(missing));
        }

        let mut instructions = Vec::new();
        for line in line_iter {
            let mut current = 0;
            for c in line.trim().chars() {
                match c {
                    'L' => {
                        if current != 0 {
                            instructions.push(Instruction::Move(current));
                            current = 0;
                        }
                        instructions.push(Instruction::Left);
                    }
                    'R' => {
                        if current != 0 {
                            instructions.push(Instruction::Move(current));
                            current = 0;
                        }
                        instructions.push(Instruction::Right);
                    }
                    c if c.is_ascii_digit() => {
                        current = current * 10 + c.to_digit(10).unwrap() as u8
                    }
                    _ => unreachable!(),
                }
            }
            if current != 0 {
                instructions.push(Instruction::Move(current));
            }
        }

        let grid = Grid {
            data,
            width,
            height,
        };

        Input { grid, instructions }
    }
}

impl Solution<Day22> for Aoc2022 {
    type Part1Output = usize;
    type Part2Output = u32;

    fn part1(input: &Input) -> usize {
        let deltas = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let mut di = 0;

        let mut x = 0;
        let mut y = 0;

        while input.grid.get(x, y).is_none() {
            x += 1;
        }

        for inst in &input.instructions {
            let (dx, dy) = deltas[di];
            match inst {
                Instruction::Move(offset) => {
                    for _ in 0..*offset {
                        let (nx, ny) = compute_offset(&input.grid, x, y, dx, dy);
                        if let Some(Cell::Wall) = *input.grid.get(nx, ny) {
                            break;
                        }
                        x = nx;
                        y = ny;
                    }
                }
                Instruction::Left => di = wrap_dec(di, deltas.len()),
                Instruction::Right => di = (di + 1) % deltas.len(),
            }
        }

        (y + 1) * 1000 + (x + 1) * 4 + di
    }

    fn part2(_input: &Input) -> u32 {
        let test_map = [[0, 0, 1, 0], [2, 3, 4, 0], [0, 0, 5, 6]];

        // let subgrids = vec![Grid<]

        todo!()
    }
}

fn compute_offset(
    grid: &Grid<Option<Cell>>,
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
) -> (usize, usize) {
    let mut nx = x;
    let mut ny = y;
    loop {
        assert!(dx.abs() <= 1);
        assert!(dy.abs() <= 1);

        if nx == 0 && dx < 0 {
            nx = grid.width;
        }
        if ny == 0 && dy < 0 {
            ny = grid.height;
        }

        nx = nx.wrapping_add_signed(dx) % grid.width;
        ny = ny.wrapping_add_signed(dy) % grid.height;

        if grid.get(nx, ny).is_some() {
            break;
        }
    }

    (nx, ny)
}

fn wrap_dec(i: usize, max: usize) -> usize {
    if i == 0 {
        max - 1
    } else {
        i - 1
    }
}
