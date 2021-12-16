use crate::aoc2021::Aoc2021;
use advent_of_code_traits::days::Day16;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;

impl ParseInput<Day16> for Aoc2021 {
    type Parsed = Vec<u8>;

    fn parse_input(input: &str) -> Vec<u8> {
        input
            .trim()
            .chars()
            .map(|c| c.to_digit(16).unwrap() as u8)
            .collect()
    }
}

#[derive(Debug)]
struct State {
    bits: Vec<u8>,
    cursor: usize,
}

fn bits_to_u32(bits: &[u8]) -> u32 {
    let mut res = 0;
    for &c in bits {
        res = (res << 1) + c as u32;
    }
    res
}

impl State {
    fn new(input: &[u8]) -> Self {
        let bits = input
            .iter()
            .flat_map(|c| format!("{:04b}", c).bytes().collect::<Vec<_>>())
            .map(|b| match b {
                b'0' => 0,
                b'1' => 1,
                _ => unreachable!(),
            })
            .collect();
        State { bits, cursor: 0 }
    }

    fn read_packet(&mut self) -> (usize, Packet) {
        let start_cursor = self.cursor;

        let version = self.read(3);
        let type_id = self.read(3);
        let body = if type_id == 4 {
            let mut value = 0;
            loop {
                let block_header = self.read(1);
                let block_value = self.read(4);
                value = (value << 4) + block_value as u64;
                if block_header == 0 {
                    break;
                }
            }
            PacketBody::Literal(value)
        } else {
            let length_type_id = self.read(1);
            let length = match length_type_id {
                0 => self.read(15),
                1 => self.read(11),
                _ => unreachable!(),
            };

            let mut subs = Vec::new();
            match length_type_id {
                0 => {
                    let mut bits = 0;
                    while bits < length {
                        let (size, p) = self.read_packet();
                        bits += size as u32;
                        subs.push(p);
                    }
                }
                1 => {
                    for _ in 0..length {
                        let (_, p) = self.read_packet();
                        subs.push(p);
                    }
                }
                _ => unreachable!(),
            };
            PacketBody::Operator(type_id, subs)
        };

        (self.cursor - start_cursor, Packet { version, body })
    }

    fn read(&mut self, bits: usize) -> u32 {
        let res = bits_to_u32(&self.bits[self.cursor..self.cursor + bits]);
        self.cursor += bits;
        res
    }
}

#[derive(Debug)]
struct Packet {
    version: u32,
    body: PacketBody,
}

impl Packet {
    fn sum_versions(&self) -> u32 {
        match &self.body {
            PacketBody::Literal(_) => self.version,
            PacketBody::Operator(_, subs) => {
                self.version + subs.iter().map(Packet::sum_versions).sum::<u32>()
            }
        }
    }

    fn eval(&self) -> u64 {
        match &self.body {
            PacketBody::Literal(lit) => *lit,
            PacketBody::Operator(0, subs) => subs.iter().map(Packet::eval).sum(),
            PacketBody::Operator(1, subs) => subs.iter().map(Packet::eval).product(),
            PacketBody::Operator(2, subs) => subs.iter().map(Packet::eval).min().unwrap(),
            PacketBody::Operator(3, subs) => subs.iter().map(Packet::eval).max().unwrap(),
            PacketBody::Operator(5, subs) => {
                assert_eq!(subs.len(), 2);
                if subs[0].eval() > subs[1].eval() {
                    1
                } else {
                    0
                }
            }
            PacketBody::Operator(6, subs) => {
                assert_eq!(subs.len(), 2);
                if subs[0].eval() < subs[1].eval() {
                    1
                } else {
                    0
                }
            }
            PacketBody::Operator(7, subs) => {
                assert_eq!(subs.len(), 2);
                if subs[0].eval() == subs[1].eval() {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum PacketBody {
    Literal(u64),
    Operator(u32, Vec<Packet>),
}

impl Solution<Day16> for Aoc2021 {
    type Part1Output = u32;
    type Part2Output = u64;

    fn part1(input: &Vec<u8>) -> u32 {
        let mut bits = State::new(input);
        let (_, p) = bits.read_packet();
        p.sum_versions()
    }

    fn part2(input: &Vec<u8>) -> u64 {
        let mut bits = State::new(input);
        let (_, p) = bits.read_packet();
        p.eval()
    }
}
