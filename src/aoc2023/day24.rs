use nalgebra::matrix;
use nalgebra::vector;

use crate::aoc2023::Aoc2023;
use crate::traits::days::Day24;
use crate::traits::ParseInput;
use crate::traits::Solution;

pub type Vec3 = nalgebra::Vector3<f64>;
pub type Vec6 = nalgebra::Vector6<f64>;
pub type Mat3 = nalgebra::Matrix3<f64>;
pub type Mat6 = nalgebra::Matrix6<f64>;

#[derive(Debug, Clone, Copy)]
pub struct Ball {
    pos: Vec3,
    speed: Vec3,
}

impl Ball {
    fn pos_cross_speed(&self) -> Vec3 {
        self.pos.cross(&self.speed)
    }
}

impl ParseInput<Day24> for Aoc2023 {
    type Parsed = Vec<Ball>;

    fn parse_input(input: &str) -> Self::Parsed {
        input
            .lines()
            .map(|line| {
                let (pos, speed) = line.split_once(" @ ").unwrap();
                let mut pos = pos.split(',').map(|val| val.trim().parse().unwrap());
                let px = pos.next().unwrap();
                let py = pos.next().unwrap();
                let pz = pos.next().unwrap();
                let pos = vector![px, py, pz];

                let mut speed = speed.split(',').map(|val| val.trim().parse().unwrap());
                let vx = speed.next().unwrap();
                let vy = speed.next().unwrap();
                let vz = speed.next().unwrap();
                let speed = vector![vx, vy, vz];

                Ball { pos, speed }
            })
            .collect()
    }
}

impl Solution<Day24> for Aoc2023 {
    type Part1Output = usize;
    type Part2Output = u64;

    fn part1(input: &Vec<Ball>) -> usize {
        const MIN: f64 = 200000000000000.0;
        const MAX: f64 = 400000000000000.0;
        let bounds = MIN..=MAX;

        let mut counter = 0;
        for (ia, a) in input.iter().enumerate() {
            for b in &input[ia + 1..] {
                let t = ((a.pos.x - b.pos.x) * -b.speed.y - (a.pos.y - b.pos.y) * -b.speed.x)
                    / (-a.speed.x * -b.speed.y - -a.speed.y * -b.speed.x);
                let u = ((a.pos.x - b.pos.x) * -a.speed.y - (a.pos.y - b.pos.y) * -a.speed.x)
                    / (-a.speed.x * -b.speed.y - -a.speed.y * -b.speed.x);

                if t < 0.0 || u < 0.0 {
                    continue;
                }

                let interx = a.pos.x + t * a.speed.x;
                let intery = a.pos.y + t * a.speed.y;

                if bounds.contains(&interx) && bounds.contains(&intery) {
                    counter += 1;
                }
            }
        }
        counter
    }

    fn part2(input: &Vec<Ball>) -> u64 {
        assert!(input.len() >= 3);

        let mut rhs = Vec6::zeros();
        let first_rows = input[1].pos_cross_speed() - input[0].pos_cross_speed();
        let last_rows = input[2].pos_cross_speed() - input[0].pos_cross_speed();
        rhs.fixed_rows_mut::<3>(0).copy_from(&first_rows);
        rhs.fixed_rows_mut::<3>(3).copy_from(&last_rows);

        let mut big_mat = Mat6::zeros();
        big_mat
            .fixed_view_mut::<3, 3>(0, 0)
            .copy_from(&(cross_mat(input[0].speed) - cross_mat(input[1].speed)));
        big_mat
            .fixed_view_mut::<3, 3>(3, 0)
            .copy_from(&(cross_mat(input[0].speed) - cross_mat(input[2].speed)));
        big_mat
            .fixed_view_mut::<3, 3>(0, 3)
            .copy_from(&(cross_mat(input[1].pos) - cross_mat(input[0].pos)));
        big_mat
            .fixed_view_mut::<3, 3>(3, 3)
            .copy_from(&(cross_mat(input[2].pos) - cross_mat(input[0].pos)));

        let res = big_mat.try_inverse().unwrap() * rhs;

        let mut sum = 0;
        for i in 0..3 {
            sum += res[i].round() as u64;
        }

        sum
    }
}

fn cross_mat(v: Vec3) -> Mat3 {
    matrix![0.0, -v[2], v[1]; v[2], 0.0, -v[0]; -v[1], v[0], 0.0]
}
