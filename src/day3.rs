use std::ops::Deref;

use advent_of_code_traits::{days::Day3, ParseInput, Solution};

use crate::aoc::Aoc2021;

#[derive(Debug)]
pub struct Bits {
    pub line_count: usize,
    pub bits_per_line: usize,
    pub bits: Vec<u8>,
}

impl Bits {
    pub fn get_bit(&self, x: usize, y: usize) -> u8 {
        self.bits[y * self.bits_per_line + x]
    }

    pub fn most_common_at(&self, x: usize, mask: Option<&[bool]>) -> Option<u8> {
        let mut count0 = 0;
        let mut count1 = 0;

        for y in 0..self.line_count {
            if let Some(mask) = mask {
                if !mask[y] {
                    continue;
                }
            }

            let value = self.get_bit(x, y);
            if value == 0 {
                count0 += 1;
            } else {
                count1 += 1;
            }
        }

        if count1 > count0 {
            Some(1)
        } else if count0 > count1 {
            Some(0)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct MaskedBits<'b> {
    pub bits: &'b Bits,
    pub line_mask: Vec<bool>,
}

impl<'b> Deref for MaskedBits<'b> {
    type Target = Bits;

    fn deref(&self) -> &Bits {
        self.bits
    }
}

impl<'b> MaskedBits<'b> {
    pub fn new(bits: &'b Bits) -> Self {
        Self {
            bits,
            line_mask: vec![true; bits.line_count],
        }
    }

    pub fn most_common_at(&self, x: usize) -> Option<u8> {
        self.bits.most_common_at(x, Some(&self.line_mask))
    }

    pub fn count_in_mask(&self) -> usize {
        self.line_mask.iter().filter(|v| **v).count()
    }

    pub fn result_masked_value(&self) -> u32 {
        assert_eq!(self.count_in_mask(), 1);

        for y in 0..self.line_count {
            if self.line_mask[y] {
                let mut value: u32 = 0;
                for x in 0..self.bits_per_line {
                    let bit = self.get_bit(x, y);
                    value = (value << 1) | (bit as u32);
                }
                return value;
            }
        }
        unreachable!()
    }
}

impl ParseInput<Day3> for Aoc2021 {
    type Parsed = Bits;

    fn parse_input(input: &str) -> Self::Parsed {
        let mut bits = Vec::new();

        let mut bits_per_line = None;
        let mut line_count = 0;

        for line in input.lines() {
            let line = line.trim();
            if let Some(bpl) = bits_per_line {
                assert_eq!(bpl, line.len());
            } else {
                bits_per_line = Some(line.len());
            }

            for c in line.bytes() {
                match c {
                    b'0' => bits.push(0),
                    b'1' => bits.push(1),
                    _ => panic!("unknown char"),
                }
            }
            line_count += 1;
        }

        Bits {
            line_count,
            bits_per_line: bits_per_line.unwrap_or_default(),
            bits,
        }
    }
}

impl Solution<Day3> for Aoc2021 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Bits) -> u32 {
        let mut gamma = 0;
        let mut epsilon = 0;

        for x in 0..input.bits_per_line {
            let most_common = input.most_common_at(x, None).expect("equal count");

            gamma <<= 1;
            epsilon <<= 1;
            if most_common == 1 {
                gamma |= 1;
            } else {
                epsilon |= 1;
            }
        }

        gamma * epsilon
    }

    fn part2(input: &Bits) -> u32 {
        fn work(input: &Bits, keep_most_common: bool, equal_keep: u8) -> u32 {
            let mut input = MaskedBits::new(input);

            for x in 0..input.bits_per_line {
                if input.count_in_mask() == 1 {
                    break;
                }

                let most_common = input.most_common_at(x);

                for y in 0..input.line_count {
                    if let Some(most_common) = most_common {
                        if keep_most_common {
                            if input.get_bit(x, y) != most_common {
                                input.line_mask[y] = false;
                            }
                        } else {
                            if input.get_bit(x, y) == most_common {
                                input.line_mask[y] = false;
                            }
                        }
                    } else {
                        if input.get_bit(x, y) != equal_keep {
                            input.line_mask[y] = false;
                        }
                    }
                }
            }
            input.result_masked_value()
        }

        let oxygen = work(input, true, 1);
        let scrubber = work(input, false, 0);

        oxygen * scrubber
    }
}
