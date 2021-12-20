use crate::aoc2021::day19::Vec3;
use nalgebra::vector;

pub fn rot0(point: Vec3) -> Vec3 {
    vector![point[0], point[1], point[2]]
}

pub fn rot1(point: Vec3) -> Vec3 {
    vector![-point[2], -point[1], -point[0]]
}

pub fn rot2(point: Vec3) -> Vec3 {
    vector![point[0], -point[1], -point[2]]
}

pub fn rot3(point: Vec3) -> Vec3 {
    vector![point[2], point[1], -point[0]]
}

pub fn rot4(point: Vec3) -> Vec3 {
    vector![-point[0], point[1], -point[2]]
}

pub fn rot5(point: Vec3) -> Vec3 {
    vector![point[2], -point[1], point[0]]
}

pub fn rot6(point: Vec3) -> Vec3 {
    vector![-point[0], -point[1], point[2]]
}

pub fn rot7(point: Vec3) -> Vec3 {
    vector![-point[2], point[1], point[0]]
}

pub fn rot8(point: Vec3) -> Vec3 {
    vector![point[1], point[2], point[0]]
}

pub fn rot9(point: Vec3) -> Vec3 {
    vector![-point[0], -point[2], -point[1]]
}

pub fn rot10(point: Vec3) -> Vec3 {
    vector![point[1], -point[2], -point[0]]
}

pub fn rot11(point: Vec3) -> Vec3 {
    vector![point[0], point[2], -point[1]]
}

pub fn rot12(point: Vec3) -> Vec3 {
    vector![-point[1], point[2], -point[0]]
}

pub fn rot13(point: Vec3) -> Vec3 {
    vector![point[0], -point[2], point[1]]
}

pub fn rot14(point: Vec3) -> Vec3 {
    vector![-point[1], -point[2], point[0]]
}

pub fn rot15(point: Vec3) -> Vec3 {
    vector![-point[0], point[2], point[1]]
}

pub fn rot16(point: Vec3) -> Vec3 {
    vector![point[2], point[0], point[1]]
}

pub fn rot17(point: Vec3) -> Vec3 {
    vector![-point[1], -point[0], -point[2]]
}

pub fn rot18(point: Vec3) -> Vec3 {
    vector![point[2], -point[0], -point[1]]
}

pub fn rot19(point: Vec3) -> Vec3 {
    vector![point[1], point[0], -point[2]]
}

pub fn rot20(point: Vec3) -> Vec3 {
    vector![-point[2], point[0], -point[1]]
}

pub fn rot21(point: Vec3) -> Vec3 {
    vector![point[1], -point[0], point[2]]
}

pub fn rot22(point: Vec3) -> Vec3 {
    vector![-point[2], -point[0], point[1]]
}

pub fn rot23(point: Vec3) -> Vec3 {
    vector![-point[1], point[0], point[2]]
}

pub const ROTATIONS: [fn(Vec3) -> Vec3; 24] = [
    rot0, rot1, rot2, rot3, rot4, rot5, rot6, rot7, rot8, rot9, rot10, rot11, rot12, rot13, rot14,
    rot15, rot16, rot17, rot18, rot19, rot20, rot21, rot22, rot23,
];
