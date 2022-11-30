use std::collections::VecDeque;

#[derive(Debug)]
pub struct IntCodeVM {
    pub memory: Vec<i32>,
    pub pc: usize,
    pub running: bool,
    pub input: VecDeque<i32>,
    pub output: Vec<i32>,
}

impl IntCodeVM {
    pub fn new(memory: Vec<i32>) -> Self {
        Self::with_input(memory, Vec::new())
    }

    pub fn with_input(memory: Vec<i32>, input: Vec<i32>) -> Self {
        Self {
            memory,
            pc: 0,
            running: true,
            input: input.into_iter().collect(),
            output: Vec::new(),
        }
    }

    pub fn read_addr(&self, addr: usize, mode: u32) -> i32 {
        let arg = self.memory[addr];
        self.read(arg, mode)
    }

    pub fn read(&self, arg: i32, mode: u32) -> i32 {
        match mode {
            0 => self.memory[arg as usize],
            1 => arg,
            _ => panic!("unknown mode"),
        }
    }

    pub fn write_addr(&mut self, addr: usize, mode: u32, value: i32) {
        let arg = self.memory[addr];
        self.write(arg, mode, value)
    }

    pub fn write(&mut self, arg: i32, mode: u32, value: i32) {
        match mode {
            0 => self.memory[arg as usize] = value,
            _ => panic!("unknown mode"),
        }
    }

    pub fn step(&mut self) {
        let instruction = self.memory[self.pc] as u32;

        let opcode = instruction % 100;
        let opcode_mode1 = (instruction / 100) % 10;
        let opcode_mode2 = (instruction / 1000) % 10;
        let opcode_mode3 = (instruction / 10000) % 10;

        let pc_offset = match opcode {
            1 => {
                let lhs = self.read_addr(self.pc + 1, opcode_mode1);
                let rhs = self.read_addr(self.pc + 2, opcode_mode2);
                self.write_addr(self.pc + 3, opcode_mode3, lhs + rhs);
                PCOp::Offset(4)
            }
            2 => {
                let lhs = self.read_addr(self.pc + 1, opcode_mode1);
                let rhs = self.read_addr(self.pc + 2, opcode_mode2);
                self.write_addr(self.pc + 3, opcode_mode3, lhs * rhs);
                PCOp::Offset(4)
            }
            3 => {
                let input_value = self.input.pop_front().unwrap();
                self.write_addr(self.pc + 1, opcode_mode1, input_value);
                PCOp::Offset(2)
            }
            4 => {
                let output_value = self.read_addr(self.pc + 1, opcode_mode1);
                self.output.push(output_value);
                PCOp::Offset(2)
            }
            5 => {
                let cond = self.read_addr(self.pc + 1, opcode_mode1);
                let target = self.read_addr(self.pc + 2, opcode_mode2);
                if cond != 0 {
                    PCOp::Direct(target as usize)
                } else {
                    PCOp::Offset(3)
                }
            }
            6 => {
                let cond = self.read_addr(self.pc + 1, opcode_mode1);
                let target = self.read_addr(self.pc + 2, opcode_mode2);
                if cond == 0 {
                    PCOp::Direct(target as usize)
                } else {
                    PCOp::Offset(3)
                }
            }
            7 => {
                let lhs = self.read_addr(self.pc + 1, opcode_mode1);
                let rhs = self.read_addr(self.pc + 2, opcode_mode2);
                let res = i32::from(lhs < rhs);
                self.write_addr(self.pc + 3, opcode_mode3, res);
                PCOp::Offset(4)
            }
            8 => {
                let lhs = self.read_addr(self.pc + 1, opcode_mode1);
                let rhs = self.read_addr(self.pc + 2, opcode_mode2);
                let res = i32::from(lhs == rhs);
                self.write_addr(self.pc + 3, opcode_mode3, res);
                PCOp::Offset(4)
            }
            99 => {
                self.running = false;
                PCOp::Offset(1)
            }
            _ => panic!("Unknown opcode: {}", opcode),
        };

        match pc_offset {
            PCOp::Offset(off) => self.pc += off,
            PCOp::Direct(pc) => self.pc = pc,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PCOp {
    Offset(usize),
    Direct(usize),
}
