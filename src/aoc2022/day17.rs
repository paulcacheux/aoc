use std::collections::HashMap;

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
        solve(input, 2022)
    }

    fn part2(input: &Vec<i32>) -> usize {
        solve(input, 1000000000000)
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

fn get(v: &[bool], index: usize) -> bool {
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
fn solve(input: &[i32], steps: usize) -> usize {
    let blocks = [
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
    let mut current_state = vec![Vec::new(); 7];

    let mut state_cache: HashMap<CacheKey, (usize, usize)> = HashMap::default();

    let mut b = 0;
    let mut add_h = 0;
    while b < steps {
        let bmod = b % blocks.len();

        let current_height = current_state.iter().map(Vec::len).max().unwrap();
        if add_h == 0 {
            let cache_key = CacheKey::new(wind.index, bmod, &current_state, current_height);
            if let Some((step, h)) = state_cache.get(&cache_key) {
                let dh = current_height - h;
                let ds = b - step;

                let skip_steps = (steps - step) / ds;
                b = skip_steps * ds + step;
                assert_eq!(b % blocks.len(), bmod);

                add_h += dh * (skip_steps - 1);
            } else {
                state_cache.insert(cache_key, (b, current_height));
            }
        }

        let current_block = &blocks[bmod];
        let mut height = current_height + 3;
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
        b += 1;
    }
    add_h + current_state.iter().map(Vec::len).max().unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CacheKey {
    jet_index: u32,
    block_index: u8,
    depths: [u8; 7],
}

impl CacheKey {
    #[inline]
    fn new(jet_index: usize, block_index: usize, state: &[Vec<bool>], height: usize) -> Self {
        let mut depths = [0; 7];
        for (coli, col) in state.iter().enumerate() {
            for i in (0..height).rev() {
                if !get(col, i) {
                    depths[coli] += 1;
                } else {
                    break;
                }
            }
        }

        CacheKey {
            jet_index: jet_index as _,
            block_index: block_index as _,
            depths,
        }
    }
}
