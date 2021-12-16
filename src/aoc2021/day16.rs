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
    packets: Vec<Packet>,
}

fn bits_to_u32(bits: &[u8]) -> u32 {
    let mut res = 0;
    for &c in bits {
        res = (res << 1) + c as u32;
    }
    res
}

struct ReadResult {
    size: usize,
    index: usize,
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
        State {
            bits,
            cursor: 0,
            packets: Vec::new(),
        }
    }

    fn read_packet(&mut self) -> ReadResult {
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
                        let read_res = self.read_packet();
                        bits += read_res.size as u32;
                        subs.push(read_res.index);
                    }
                }
                1 => {
                    for _ in 0..length {
                        let read_res = self.read_packet();
                        subs.push(read_res.index);
                    }
                }
                _ => unreachable!(),
            };
            PacketBody::Operator(type_id, subs)
        };

        let index = self.packets.len();
        self.packets.push(Packet { version, body });
        ReadResult {
            size: self.cursor - start_cursor,
            index,
        }
    }

    fn read(&mut self, bits: usize) -> u32 {
        let res = bits_to_u32(&self.bits[self.cursor..self.cursor + bits]);
        self.cursor += bits;
        res
    }

    fn sum_versions(&self) -> u32 {
        self.packets.iter().map(|p| p.version).sum()
    }

    fn eval(&self, packet_index: usize) -> u64 {
        let packet = &self.packets[packet_index];
        match &packet.body {
            PacketBody::Literal(lit) => *lit,
            PacketBody::Operator(0, subs) => subs.iter().map(|p| self.eval(*p)).sum(),
            PacketBody::Operator(1, subs) => subs.iter().map(|p| self.eval(*p)).product(),
            PacketBody::Operator(2, subs) => subs.iter().map(|p| self.eval(*p)).min().unwrap(),
            PacketBody::Operator(3, subs) => subs.iter().map(|p| self.eval(*p)).max().unwrap(),
            PacketBody::Operator(5, subs) => {
                assert_eq!(subs.len(), 2);
                if self.eval(subs[0]) > self.eval(subs[1]) {
                    1
                } else {
                    0
                }
            }
            PacketBody::Operator(6, subs) => {
                assert_eq!(subs.len(), 2);
                if self.eval(subs[0]) < self.eval(subs[1]) {
                    1
                } else {
                    0
                }
            }
            PacketBody::Operator(7, subs) => {
                assert_eq!(subs.len(), 2);
                if self.eval(subs[0]) == self.eval(subs[1]) {
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
struct Packet {
    version: u32,
    body: PacketBody,
}

#[derive(Debug)]
enum PacketBody {
    Literal(u64),
    Operator(u32, Vec<usize>),
}

impl Solution<Day16> for Aoc2021 {
    type Part1Output = u32;
    type Part2Output = u64;

    fn part1(input: &Vec<u8>) -> u32 {
        let mut bits = State::new(input);
        let _ = bits.read_packet();
        bits.sum_versions()
    }

    fn part2(input: &Vec<u8>) -> u64 {
        let mut bits = State::new(input);
        let read_res = bits.read_packet();
        bits.eval(read_res.index)
    }
}
