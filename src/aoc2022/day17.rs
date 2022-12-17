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
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Vec<i32>) -> u32 {
        let blocks = vec![
            vec![(0, 1), (0, 1), (0, 1), (0, 1)], // horizontal line
            vec![(1, 1), (0, 3), (1, 1)],         // cross
            vec![(0, 1), (0, 1), (0, 3)],         // reverse L
            vec![(0, 4)],                         // vertical line
            vec![(0, 2), (0, 2)],                 // square
        ];

        let mut wind = Repeater::new(input);
        let mut current_state = [0; 7];

        for b in 0..2022 {
            let current_block = &blocks[b % blocks.len()];
            let mut height = current_state.iter().copied().max().unwrap() + 3;
            let mut xoff = 2;

            let mut state = true;

            loop {
                let new_xoff;
                let new_height;
                if state {
                    new_xoff = (xoff + wind.next()).clamp(0, 7 - current_block.len() as i32);
                    new_height = height;
                } else {
                    new_height = height - 1;
                    new_xoff = xoff;
                }
                state = !state;

                let valid = current_state
                    .iter()
                    .skip(new_xoff as usize)
                    .zip(current_block.iter().map(|p| p.0 + new_height))
                    .all(|(s, y)| y >= *s);

                if !valid {
                    for i in 0..current_block.len() {
                        current_state[xoff as usize + i] =
                            height + current_block[i].0 + current_block[i].1;
                    }
                    break;
                }
                height = new_height;
                xoff = new_xoff;
            }
            dbg!(current_state);
        }

        todo!()
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
