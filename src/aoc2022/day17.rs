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
    type Part2Output = u32;

    fn part1(input: &Vec<i32>) -> usize {
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

        const STEPS: usize = 2022;
        let mut wind = Repeater::new(input);
        let mut current_state = vec![Vec::with_capacity(STEPS * 4); 7];

        for b in 0..STEPS {
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
            // dbg_state(&current_state);
        }

        current_state.iter().map(Vec::len).max().unwrap()
    }

    fn part2(_input: &Vec<i32>) -> u32 {
        todo!()
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
