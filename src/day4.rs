use crate::aoc::Aoc2021;
use advent_of_code_traits::days::Day4;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;
use itertools::Itertools;

pub struct BingoGame {
    pub numbers: Vec<u8>,
    pub boards: Vec<BingoBoard>,
}

#[derive(Debug)]
pub struct BingoBoard {
    pub numbers: Vec<u8>,
}

pub struct MaskedBoard<'b> {
    board: &'b BingoBoard,
    mask: Vec<bool>,
    in_play: bool,
}

impl<'b> MaskedBoard<'b> {
    fn new(board: &'b BingoBoard) -> Self {
        MaskedBoard {
            board,
            mask: vec![false; 25],
            in_play: true,
        }
    }

    fn select_number(&mut self, n: u8) {
        for (curr, mask) in self.board.numbers.iter().zip(self.mask.iter_mut()) {
            if *curr == n {
                *mask = true;
            }
        }
    }

    fn is_selected(&self, x: usize, y: usize) -> bool {
        debug_assert!(x < 5);
        debug_assert!(y < 5);

        self.mask[y * 5 + x]
    }

    fn is_winning(&self) -> bool {
        // columns
        for x in 0..5 {
            let col = (0..5)
                .map(|y| self.is_selected(x, y))
                .reduce(|a, b| a && b)
                .unwrap();
            if col {
                return true;
            }
        }

        // columns
        for y in 0..5 {
            let row = (0..5)
                .map(|x| self.is_selected(x, y))
                .reduce(|a, b| a && b)
                .unwrap();
            if row {
                return true;
            }
        }
        false
    }

    fn score(&self, last_selected: u8) -> u32 {
        let mut score: u32 = 0;
        for (curr, mask) in self.board.numbers.iter().zip(self.mask.iter()) {
            if !mask {
                score += *curr as u32;
            }
        }
        score * last_selected as u32
    }
}

impl ParseInput<Day4> for Aoc2021 {
    type Parsed = BingoGame;

    fn parse_input(input: &str) -> BingoGame {
        let mut lines_iter = input.lines();

        let first_line = lines_iter.next().expect("no first line?");
        let numbers = first_line.split(',').map(|n| n.parse().unwrap()).collect();

        let boards = lines_iter
            .filter(|line| !line.trim().is_empty())
            .chunks(5)
            .into_iter()
            .map(|chunk| {
                let numbers: Vec<u8> = chunk
                    .flat_map(|line| line.split_ascii_whitespace().map(|n| n.parse().unwrap()))
                    .collect();

                assert_eq!(numbers.len(), 25);

                BingoBoard { numbers }
            })
            .collect();

        BingoGame { numbers, boards }
    }
}

impl Solution<Day4> for Aoc2021 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &BingoGame) -> u32 {
        let mut masked_boards: Vec<_> = input.boards.iter().map(MaskedBoard::new).collect();

        for n in &input.numbers {
            for m in &mut masked_boards {
                m.select_number(*n);

                if m.is_winning() {
                    return m.score(*n);
                }
            }
        }

        unreachable!()
    }

    fn part2(input: &BingoGame) -> u32 {
        let mut masked_boards: Vec<_> = input.boards.iter().map(MaskedBoard::new).collect();

        for n in &input.numbers {
            let mut in_play_count = 0;
            let mut last_masked_board = None;
            for m in &mut masked_boards {
                if !m.in_play {
                    continue;
                }
                in_play_count += 1;

                m.select_number(*n);

                if m.is_winning() {
                    m.in_play = false;
                    last_masked_board = Some(m);
                }
            }

            if in_play_count == 1 {
                return last_masked_board.unwrap().score(*n);
            }
        }

        unreachable!()
    }
}
