use crate::aoc2023::Aoc2023;
use crate::traits::days::Day15;
use crate::traits::ParseInput;
use crate::traits::Solution;

impl ParseInput<Day15> for Aoc2023 {
    type Parsed = Vec<String>;

    fn parse_input(input: &str) -> Self::Parsed {
        input.split(',').map(|part| part.to_owned()).collect()
    }
}

impl Solution<Day15> for Aoc2023 {
    type Part1Output = u64;
    type Part2Output = u64;

    fn part1(input: &Vec<String>) -> u64 {
        let mut sum = 0;
        for part in input {
            let part = part.trim();
            sum += hash_label(part);
        }
        sum
    }

    fn part2(input: &Vec<String>) -> u64 {
        let mut boxes = vec![Vec::<(&str, u64)>::new(); 256];
        for part in input {
            let part = part.trim();
            if let Some(label) = part.strip_suffix('-') {
                let label_hash = hash_label(label);
                boxes[label_hash as usize].retain(|&(cell_label, _)| label != cell_label);
            } else {
                let (label, focal) = part.split_once('=').unwrap();
                let focal = focal.parse().unwrap();
                let label_hash = hash_label(label);

                let selected_box = &mut boxes[label_hash as usize];
                let mut found = false;
                for lens in selected_box.iter_mut() {
                    if lens.0 == label {
                        *lens = (label, focal);
                        found = true;
                    }
                }
                if !found {
                    selected_box.push((label, focal));
                }
            }
        }

        let mut power_sum = 0;
        for (i, lenses) in boxes.into_iter().enumerate() {
            for (j, (_, focal)) in lenses.into_iter().enumerate() {
                let power = (i as u64 + 1) * (j as u64 + 1) * focal;
                power_sum += power;
            }
        }
        power_sum
    }
}

fn hash_label(label: &str) -> u64 {
    let mut h = Hasher::default();
    for b in label.bytes() {
        h.write(b);
    }
    h.state
}

#[derive(Debug, Default)]
struct Hasher {
    state: u64,
}

impl Hasher {
    fn write(&mut self, c: u8) {
        self.state += c as u64;
        self.state *= 17;
        self.state %= 256;
    }
}
