use ahash::HashMap;

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
    type Part2Output = usize;

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
                        let (nx, ny) = compute_offset_part1(&input.grid, x, y, dx, dy);
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

    fn part2(input: &Input) -> usize {
        // let map = [[0, 0, 1, 0], [2, 3, 4, 0], [0, 0, 5, 6]];
        let map = [[0, 1, 2], [0, 3, 0], [4, 5, 0], [6, 0, 0]];

        let subheight = input.grid.height / map.len();
        let subwidth = input.grid.width / map[0].len();
        assert_eq!(subwidth, subheight);

        let mut subgrids = vec![Grid::new(subwidth, subheight, Cell::Empty); 6];

        for (x, y, val) in input.grid.iter() {
            let gx = x / subwidth;
            let gy = y / subheight;
            let gindex = map[gy][gx];
            if gindex == 0 {
                assert!(val.is_none());
                continue;
            } else {
                assert!(val.is_some());
            }
            let gindex = gindex - 1;
            subgrids[gindex].set(x % subwidth, y % subheight, val.unwrap());
        }

        let mut state = State {
            x: 0,
            y: 0,
            g: 0,
            dx: 1,
            dy: 0,
        };
        let deltas = [(1, 0), (0, 1), (-1, 0), (0, -1)];

        for inst in &input.instructions {
            let di = deltas
                .iter()
                .position(|&d| d == (state.dx, state.dy))
                .unwrap();

            match inst {
                Instruction::Move(offset) => {
                    for _ in 0..*offset {
                        let next_state = compute_next_pos_part2(&subgrids, &state, subwidth);
                        if let Cell::Wall = *subgrids[next_state.g].get(next_state.x, next_state.y)
                        {
                            break;
                        }
                        state = next_state;
                    }
                }
                Instruction::Left => {
                    let di = wrap_dec(di, deltas.len());
                    let (dx, dy) = deltas[di];
                    state.dx = dx;
                    state.dy = dy;
                }
                Instruction::Right => {
                    let di = (di + 1) % deltas.len();
                    let (dx, dy) = deltas[di];
                    state.dx = dx;
                    state.dy = dy;
                }
            }
        }

        let mut gx = 0;
        let mut gy = 0;
        'top: for (y, line) in map.into_iter().enumerate() {
            for (x, g) in line.into_iter().enumerate() {
                if g - 1 == state.g {
                    gx = x;
                    gy = y;
                    break 'top;
                }
            }
        }

        let di = deltas
            .iter()
            .position(|&d| d == (state.dx, state.dy))
            .unwrap();

        let x = gx * subwidth + state.x;
        let y = gy * subwidth + state.y;

        (y + 1) * 1000 + (x + 1) * 4 + di
    }
}

fn compute_offset_part1(
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

#[derive(Debug)]
struct State {
    x: usize,
    y: usize,
    g: usize,
    dx: isize,
    dy: isize,
}

fn compute_next_pos_part2(grids: &[Grid<Cell>], s: &State, width: usize) -> State {
    let mut dir_map = HashMap::<
        (usize, (isize, isize)),
        (
            usize,
            (isize, isize),
            Box<dyn Fn(usize, usize) -> (usize, usize)>,
        ),
    >::default();
    /*
    test input
    dir_map.insert((0, (0, 1)), (3, (0, 1), Box::new(|x, _y| (x, 0))));
    dir_map.insert(
        (3, (1, 0)),
        (5, (0, 1), Box::new(|_x, y| (width - y - 1, 0))),
    );
    dir_map.insert((5, (-1, 0)), (4, (-1, 0), Box::new(|_x, y| (width - 1, y))));
    dir_map.insert(
        (4, (0, 1)),
        (1, (0, -1), Box::new(|x, _y| (width - x - 1, width - 1))),
    );
    dir_map.insert((1, (1, 0)), (2, (1, 0), Box::new(|_x, y| (0, y))));
    dir_map.insert((2, (0, -1)), (0, (1, 0), Box::new(|x, _y| (0, x))));
    */
    dir_map.insert((0, (0, -1)), (5, (1, 0), Box::new(|x, _y| (0, x))));
    dir_map.insert((5, (0, -1)), (3, (0, -1), Box::new(|x, _y| (x, width - 1))));
    dir_map.insert(
        (3, (-1, 0)),
        (0, (1, 0), Box::new(|_x, y| (0, width - y - 1))),
    );
    dir_map.insert((3, (0, 1)), (5, (0, 1), Box::new(|x, _y| (x, 0))));
    dir_map.insert((5, (-1, 0)), (0, (0, 1), Box::new(|_x, y| (y, 0))));
    dir_map.insert((0, (1, 0)), (1, (1, 0), Box::new(|_x, y| (0, y))));
    dir_map.insert((1, (-1, 0)), (0, (-1, 0), Box::new(|_x, y| (width - 1, y))));
    dir_map.insert((0, (0, 1)), (2, (0, 1), Box::new(|x, _y| (x, 0))));
    dir_map.insert((2, (1, 0)), (1, (0, -1), Box::new(|_x, y| (y, width - 1))));
    dir_map.insert((1, (0, 1)), (2, (-1, 0), Box::new(|x, _y| (width - 1, x))));
    dir_map.insert((2, (0, 1)), (4, (0, 1), Box::new(|x, _y| (x, 0))));
    dir_map.insert((4, (0, -1)), (2, (0, -1), Box::new(|x, _y| (x, width - 1))));
    dir_map.insert(
        (4, (1, 0)),
        (1, (-1, 0), Box::new(|_x, y| (width - 1, width - y - 1))),
    );
    dir_map.insert(
        (1, (1, 0)),
        (4, (-1, 0), Box::new(|_x, y| (width - 1, width - y - 1))),
    );
    dir_map.insert((4, (-1, 0)), (3, (-1, 0), Box::new(|_x, y| (width - 1, y))));
    dir_map.insert((3, (1, 0)), (4, (1, 0), Box::new(|_x, y| (0, y))));
    dir_map.insert((1, (0, -1)), (5, (0, -1), Box::new(|x, _y| (x, width - 1))));
    dir_map.insert((5, (0, 1)), (1, (0, 1), Box::new(|x, _y| (x, 0))));
    dir_map.insert(
        (0, (-1, 0)),
        (3, (1, 0), Box::new(|_x, y| (0, width - 1 - y))),
    );
    dir_map.insert((5, (1, 0)), (4, (0, -1), Box::new(|_x, y| (y, width - 1))));
    dir_map.insert((4, (0, 1)), (5, (-1, 0), Box::new(|x, _y| (width - 1, x))));
    dir_map.insert((3, (0, -1)), (2, (1, 0), Box::new(|x, _y| (0, x))));
    dir_map.insert((2, (0, -1)), (0, (0, -1), Box::new(|x, _y| (x, width - 1))));
    dir_map.insert((2, (-1, 0)), (3, (0, 1), Box::new(|_x, y| (y, 0))));

    assert!(s.dx.abs() <= 1);
    assert!(s.dy.abs() <= 1);

    if (s.x == 0 && s.dx < 0)
        || (s.y == 0 && s.dy < 0)
        || (s.x.wrapping_add_signed(s.dx) == grids[s.g].width)
        || (s.y.wrapping_add_signed(s.dy) == grids[s.g].height)
    {
        if let Some((newg, (newdx, newdy), xymapper)) = dir_map.get(&(s.g, (s.dx, s.dy))) {
            let (x, y) = xymapper(s.x, s.y);
            State {
                x,
                y,
                g: *newg,
                dx: *newdx,
                dy: *newdy,
            }
        } else {
            dbg!(s);
            unimplemented!()
        }
    } else {
        State {
            x: s.x.wrapping_add_signed(s.dx),
            y: s.y.wrapping_add_signed(s.dy),
            ..*s
        }
    }
}
