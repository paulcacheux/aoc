use crate::aoc2024::Aoc2024;
use crate::traits::days::Day5;
use crate::traits::ParseInput;
use crate::traits::Solution;

impl ParseInput<Day5> for Aoc2024 {
    type Parsed = Input;

    fn parse_input(input: &str) -> Self::Parsed {
        let mut is_in_pages = false;
        let mut res = Input::default();

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                is_in_pages = true;
                continue;
            }

            if !is_in_pages {
                let (left, right) = line.split_once('|').unwrap();
                let left = left.parse().unwrap();
                let right = right.parse().unwrap();
                let pair = (left, right);
                res.pairs.push(pair);
            } else {
                let page = line.split(',').map(|s| s.parse().unwrap()).collect();
                res.pages.push(page);
            }
        }

        res
    }
}
#[derive(Debug, Default)]
pub struct Input {
    pairs: Vec<(u32, u32)>,
    pages: Vec<Vec<u32>>,
}

impl Solution<Day5> for Aoc2024 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Input) -> u32 {
        let mut res = 0;

        'page: for page in &input.pages {
            for (i, left) in page.iter().enumerate() {
                for right in page[i + 1..].iter() {
                    let rev_pair = (*right, *left);
                    if input.pairs.contains(&rev_pair) {
                        continue 'page;
                    }
                }
            }

            res += page[page.len() / 2];
        }
        res
    }

    fn part2(input: &Input) -> u32 {
        let mut wrong_pages = Vec::new();

        'page: for page in &input.pages {
            for (i, left) in page.iter().enumerate() {
                for right in page[i + 1..].iter() {
                    let rev_pair = (*right, *left);
                    if input.pairs.contains(&rev_pair) {
                        wrong_pages.push(page);
                        continue 'page;
                    }
                }
            }
        }

        let mut res = 0;
        for page in wrong_pages {
            let mut page = page.clone();
            page.sort_by(|&a, &b| {
                if input.pairs.contains(&(a, b)) {
                    std::cmp::Ordering::Less
                } else if input.pairs.contains(&(b, a)) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            res += page[page.len() / 2];
        }

        res
    }
}
