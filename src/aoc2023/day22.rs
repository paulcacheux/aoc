use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

use crate::aoc2023::Aoc2023;
use crate::traits::days::Day22;
use crate::traits::ParseInput;
use crate::traits::Solution;

#[derive(Debug, Clone, Copy)]
pub struct Brick {
    start: Point,
    end: Point,
}

impl Brick {
    fn collide(a: &Self, b: &Self) -> bool {
        let xstart = a.start.x.max(b.start.x);
        let xend = a.end.x.min(b.end.x);

        let ystart = a.start.y.max(b.start.y);
        let yend = a.end.y.min(b.end.y);

        let zstart = a.start.z.max(b.start.z);
        let zend = a.end.z.min(b.end.z);

        xstart <= xend && ystart <= yend && zstart <= zend
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl From<(usize, usize, usize)> for Point {
    fn from((x, y, z): (usize, usize, usize)) -> Self {
        Self { x, y, z }
    }
}

impl ParseInput<Day22> for Aoc2023 {
    type Parsed = Vec<Brick>;

    fn parse_input(input: &str) -> Self::Parsed {
        input
            .lines()
            .map(|line| {
                let (start, end) = line.split_once('~').unwrap();
                let start: (usize, usize, usize) = start
                    .split(',')
                    .map(|val| val.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                let end: (usize, usize, usize) = end
                    .split(',')
                    .map(|val| val.parse().unwrap())
                    .collect_tuple()
                    .unwrap();

                let start: Point = start.into();
                let end: Point = end.into();

                assert!(start.x <= end.x);
                assert!(start.y <= end.y);
                assert!(start.z <= end.z);

                Brick { start, end }
            })
            .collect()
    }
}

impl Solution<Day22> for Aoc2023 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(input: &Vec<Brick>) -> usize {
        let support = compute_support_chain(input);
        let mut potential_supports = vec![true; input.len()];

        for js in support.values() {
            if js.len() == 1 {
                potential_supports[js[0]] = false;
            }
        }

        potential_supports.into_iter().filter(|&v| v).count()
    }

    fn part2(input: &Vec<Brick>) -> usize {
        let support = compute_support_chain(input);

        // compute roots
        let mut potential_supports = vec![true; input.len()];
        for js in support.values() {
            if js.len() == 1 {
                potential_supports[js[0]] = false;
            }
        }
        let roots: Vec<_> = potential_supports
            .into_iter()
            .enumerate()
            .filter_map(|(i, v)| if !v { Some(i) } else { None })
            .collect();
        
        let mut leafs: HashMap<usize, HashSet<usize>> = HashMap::new();

        let mut open_queue = Vec::new();
        for (&main, unders) in &support {
            for &under in unders {
                open_queue.push((main, under));
            }
        }

        while let Some((main, brick)) = open_queue.pop() {
            let unders = support.get(&brick);

            if roots.contains(&brick) {
                leafs.entry(main).or_default().insert(brick);
            }

            if let Some(unders) = unders {
                assert!(!unders.is_empty());

                for &under in unders {
                    open_queue.push((main, under));
                }
            }
        }

        let mut counter = 0;

        for roots in leafs.values() {
            let mut meta_roots = HashSet::new();

            let mut open_roots: Vec<_> = roots.iter().copied().collect();
            for root in roots {
                if let Some(subroot) = leafs.get(root) {
                    open_roots.extend(subroot.iter().copied());
                } else {
                    meta_roots.insert(*root);
                }
            }

            if meta_roots.len() == 1 {
                counter += roots.len()
            } else {
                dbg!(roots, meta_roots);
            }
        }

        counter
    }
}

fn compute_support_chain(bricks: &Vec<Brick>) -> HashMap<usize, Vec<usize>> {
    let mut bricks = bricks.clone();
    bricks.sort_by_key(|b| (b.start.z, b.end.z));

    let mut support: HashMap<usize, Vec<usize>> = HashMap::new();

    for i in 0..bricks.len() {
        loop {
            if bricks[i].start.z == 0 {
                break;
            }

            let mut new_brick = bricks[i];
            new_brick.start.z -= 1;
            new_brick.end.z -= 1;

            let mut valid = true;
            for (j, under) in bricks[..i].iter().enumerate() {
                if Brick::collide(&new_brick, under) {
                    support.entry(i).or_default().push(j);
                    valid = false;
                }
            }

            if !valid {
                break;
            }

            bricks[i] = new_brick;
        }
    }

    support
}
