#[derive(Debug)]
pub struct IntCodeVM {
    pub memory: Vec<i32>,
    pub pc: usize,
    pub running: bool,
}

impl IntCodeVM {
    pub fn new(memory: Vec<i32>) -> Self {
        Self {
            memory,
            pc: 0,
            running: true,
        }
    }

    pub fn step(&mut self) {
        let opcode = self.memory[self.pc];
        let pc_offset = match opcode {
            1 => {
                let lhs = self.memory[self.pc + 1];
                let rhs = self.memory[self.pc + 2];
                let dest = self.memory[self.pc + 3];

                let lhs = self.memory[lhs as usize];
                let rhs = self.memory[rhs as usize];
                self.memory[dest as usize] = lhs + rhs;
                4
            }
            2 => {
                let lhs = self.memory[self.pc + 1];
                let rhs = self.memory[self.pc + 2];
                let dest = self.memory[self.pc + 3];

                let lhs = self.memory[lhs as usize];
                let rhs = self.memory[rhs as usize];
                self.memory[dest as usize] = lhs * rhs;
                4
            }
            99 => {
                self.running = false;
                1
            }
            _ => panic!("Unknown opcode: {}", opcode),
        };
        self.pc += pc_offset;
    }
}
