use crate::aoc2022::Aoc2022;
use crate::traits::days::Day20;
use crate::traits::ParseInput;
use crate::traits::Solution;

impl ParseInput<Day20> for Aoc2022 {
    type Parsed = Vec<i32>;

    fn parse_input(input: &str) -> Self::Parsed {
        input
            .split_ascii_whitespace()
            .map(|w| w.parse().unwrap())
            .collect()
    }
}

impl Solution<Day20> for Aoc2022 {
    type Part1Output = i32;
    type Part2Output = i32;

    fn part1(input: &Vec<i32>) -> i32 {
        let mut cycle = Cycle::new(input, None);
        mix(&mut cycle);
        cycle.result()
    }

    fn part2(input: &Vec<i32>) -> i32 {
        const DEC_KEY: i32 = 811589153;
        let mut cycle = Cycle::new(input, Some(DEC_KEY));

        for i in 0..10 {
            println!("turn {i}");
            mix(&mut cycle);
        }
        cycle.result()
    }
}

fn mix(cycle: &mut Cycle) {
    for i in 0..cycle.inner.len() {
        let (j, item) = cycle
            .inner
            .iter()
            .copied()
            .enumerate()
            .find(|&(_, item)| item.index == i)
            .unwrap();

        for offset in 0..item.index_val {
            let t = cycle.get(j, offset + 1);
            cycle.set(j, offset, t);
        }
        cycle.set(j, item.index_val, item);
    }
}

fn compute_new_index(base: usize, offset: i32, len: usize) -> usize {
    if offset >= 0 {
        return (base + offset as usize) % len;
    }

    (base as i64 + offset as i64).rem_euclid(len as _) as usize
}

struct Cycle {
    inner: Vec<CycleItem>,
}

#[derive(Debug, Clone, Copy)]
struct CycleItem {
    index: usize,
    val: i32,
    index_val: i32,
}

impl Cycle {
    fn new(input: &[i32], multiply: Option<i32>) -> Self {
        Cycle {
            inner: input
                .iter()
                .copied()
                .enumerate()
                .map(|(index, val)| {
                    let val = if let Some(m) = multiply { val * m } else { val };

                    let index_val = if val >= 0 {
                        val
                    } else {
                        let mut fake_val = val;
                        while fake_val < 0 {
                            fake_val += input.len() as i32;
                        }
                        fake_val -= 1;
                        fake_val
                    };

                    CycleItem {
                        index,
                        val,
                        index_val,
                    }
                })
                .collect(),
        }
    }

    fn get(&self, base: usize, offset: i32) -> CycleItem {
        self.inner[compute_new_index(base, offset, self.inner.len())]
    }

    fn set(&mut self, base: usize, offset: i32, new_val: CycleItem) {
        let len = self.inner.len();
        self.inner[compute_new_index(base, offset, len)] = new_val;
    }

    fn result(&self) -> i32 {
        let (index0, _) = self
            .inner
            .iter()
            .copied()
            .enumerate()
            .find(|(_, p)| p.val == 0)
            .unwrap();

        let a = self.get(index0, 1000).val;
        let b = self.get(index0, 2000).val;
        let c = self.get(index0, 3000).val;
        a + b + c
    }
}
