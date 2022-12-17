use crate::aoc2022::Aoc2022;
use crate::traits::days::Day17;
use crate::traits::ParseInput;
use crate::traits::Solution;

impl ParseInput<Day17> for Aoc2022 {
    type Parsed = Vec<i32>;

    fn parse_input(input: &str) -> Self::Parsed {
        input
            .trim()
            .chars()
            .filter_map(|c| match c {
                '<' => Some(-1),
                '>' => Some(1),
                _ => None,
            })
            .collect()
    }
}

impl Solution<Day17> for Aoc2022 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Vec<i32>) -> usize {
        solve_part1(input, 2022, None)
    }

    fn part2(input: &Vec<i32>) -> usize {
        let base_period = input.len() * 5; // wind period * block period
        let mut heights = Vec::new();
        solve_part1(input, base_period * 4096, Some(&mut heights));

        dbg!("found heights");

        let mut per_heights = heights.clone();
        const STABLE: usize = 20;

        let mut best_factor = None;
        for factor in (1..128).rev() {
            let mut current_bottom = 0;
            for (i, h) in per_heights.iter_mut().enumerate() {
                if i % (base_period * factor) == 0 {
                    current_bottom = *h;
                }
                *h -= current_bottom;
            }

            // skip the beginning
            if per_heights[base_period * factor * STABLE..base_period * factor * (STABLE + 1)]
                == per_heights
                    [base_period * factor * (STABLE + 1)..base_period * factor * (STABLE + 2)]
                && per_heights
                    [base_period * factor * (STABLE + 1)..base_period * factor * (STABLE + 2)]
                    == per_heights
                        [base_period * factor * (STABLE + 2)..base_period * factor * (STABLE + 3)]
                && heights[base_period * factor * (STABLE + 2)]
                    - heights[base_period * factor * (STABLE + 1)]
                    == heights[base_period * factor * (STABLE + 1)]
                        - heights[base_period * factor * STABLE]
            {
                best_factor = Some(factor);
                break;
            }
        }
        let best_factor = best_factor.unwrap();

        const GOAL: usize = 1000000000000 - 1; // -1 to go to 0-index

        let offset = GOAL % (base_period * best_factor);
        let coeff = GOAL / (base_period * best_factor);
        let res = heights[base_period * best_factor * STABLE + offset]
            + (heights[base_period * best_factor * (STABLE + 1)]
                - heights[base_period * best_factor * STABLE])
                * (coeff - STABLE);
        res
    }
}

struct Repeater<'a, T: Copy> {
    inner: &'a [T],
    index: usize,
}

impl<'a, T: Copy> Repeater<'a, T> {
    fn new(inner: &'a [T]) -> Self {
        Self { inner, index: 0 }
    }

    fn next(&mut self) -> T {
        let val = self.inner[self.index];
        self.index = (self.index + 1) % self.inner.len();
        val
    }
}

fn set(v: &mut Vec<bool>, index: usize) {
    if index >= v.len() {
        v.resize(index + 1, false);
    }
    v[index] = true;
}

fn get(v: &Vec<bool>, index: usize) -> bool {
    if index >= v.len() {
        false
    } else {
        v[index]
    }
}

#[allow(dead_code)]
fn dbg_state(state: &Vec<Vec<bool>>) {
    let height = state.iter().map(Vec::len).max().unwrap();

    for i in (0..height).rev() {
        for col in state {
            if get(col, i) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }
    println!()
}

#[inline]
fn solve_part1(input: &[i32], steps: usize, mut heights: Option<&mut Vec<usize>>) -> usize {
    let blocks = vec![
        vec![
            [true, false, false, false],
            [true, false, false, false],
            [true, false, false, false],
            [true, false, false, false],
        ], // horizontal line
        vec![
            [false, true, false, false],
            [true, true, true, false],
            [false, true, false, false],
        ], // cross
        vec![
            [true, false, false, false],
            [true, false, false, false],
            [true, true, true, false],
        ], // reverse L
        vec![[true, true, true, true]], // vertical line
        vec![[true, true, false, false], [true, true, false, false]], // square
    ];

    let mut wind = Repeater::new(input);
    let mut current_state = vec![Vec::with_capacity(steps * 4); 7];

    for b in 0..steps {
        let current_block = &blocks[b % blocks.len()];
        let mut height = current_state.iter().map(Vec::len).max().unwrap() + 3;
        let mut xoff = 2;

        let mut step_state = true;

        loop {
            let new_xoff;
            let new_height;
            let mut bottom = false;
            if step_state {
                new_xoff = (xoff + wind.next()).clamp(0, 7 - current_block.len() as i32);
                new_height = height;
            } else {
                if height > 0 {
                    new_height = height - 1;
                } else {
                    new_height = height;
                    bottom = true;
                }
                new_xoff = xoff;
            }

            let mut valid = true; // allow detection of bottom
            'main: for i in 0..current_block.len() {
                for y in 0..4 {
                    if current_block[i][y]
                        && get(&current_state[new_xoff as usize + i], new_height + y)
                    {
                        valid = false;
                        break 'main;
                    }
                }
            }

            if (!valid || bottom) && !step_state {
                for i in 0..current_block.len() {
                    for y in 0..4 {
                        if current_block[i][y] {
                            set(&mut current_state[xoff as usize + i], height + y);
                        }
                    }
                }
                break;
            }
            if valid {
                height = new_height;
                xoff = new_xoff;
            }
            step_state = !step_state;
        }

        if let Some(h) = &mut heights {
            h.push(current_state.iter().map(Vec::len).max().unwrap());
        }
    }
    current_state.iter().map(Vec::len).max().unwrap()
}
