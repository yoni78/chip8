use crate::display::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

const MEM_SIZE: usize = 4 * 1024;
const PROGRAM_LOC: usize = 0x200;
const REGS_COUNT: usize = 16;

pub struct Emulator {
    memory: Vec<u8>,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    pc: u16,
    index: u16,
    regs: [u8; REGS_COUNT],
    pub display: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            memory: vec![0; MEM_SIZE],
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            pc: 0,
            index: 0,
            regs: [0; REGS_COUNT],
            display: [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        }
    }

    pub fn execute_program(&mut self, program_data: &[u8]) {
        self.load_program(program_data);
        self.pc = PROGRAM_LOC as u16;
    }

    pub fn execute_next_instruction(&mut self) {
        let inst = self.fetch();
        self.execute(inst);
    }

    fn load_program(&mut self, program_data: &[u8]) {
        for (i, byte) in program_data.iter().enumerate() {
            self.memory[PROGRAM_LOC + i] = *byte;
        }
    }

    fn fetch(&mut self) -> u16 {
        // TODO: Check PC bounds
        let msb = self.memory[self.pc as usize];
        let lsb = self.memory[(self.pc + 1) as usize];

        self.pc += 2;

        let mut inst: u16 = 0;

        inst += msb as u16;
        inst <<= 8;
        inst += lsb as u16;

        return inst;
    }

    fn execute(&mut self, inst: u16) {
        let op = Emulator::get_opcode(inst);

        match op {
            0x0 => match inst {
                0x00e0 => self.clear_screen(),
                0x00ee => self.subroutine_ret(),

                _ => {}
            },

            0x1 => self.jump(inst),
            0x2 => self.subroutine_call(inst),
            0x3 | 0x4 | 0x5 | 0x9 => self.skip(inst),
            0x6 => self.set(inst),
            0x7 => self.add(inst),
            0xa => self.set_index(inst),
            0xd => self.display(inst),

            // TODO: Invalid instruction error
            _ => {}
        }
    }

    fn get_opcode(inst: u16) -> u16 {
        (inst & 0xf000) >> 12
    }

    fn get_first_reg(inst: u16) -> u8 {
        ((inst & 0x0f00) >> 8) as u8
    }

    fn get_second_reg(inst: u16) -> u8 {
        ((inst & 0x00f0) >> 4) as u8
    }

    fn get_immediate_number(inst: u16) -> u8 {
        (inst & 0x000f) as u8
    }

    fn get_double_immediate_number(inst: u16) -> u8 {
        (inst & 0x00ff) as u8
    }

    fn get_immediate_addr(inst: u16) -> u16 {
        inst & 0x0fff
    }

    fn clear_screen(&mut self) {
        for i in 0..DISPLAY_HEIGHT {
            for j in 0..DISPLAY_WIDTH {
                self.display[i][j] = 0;
            }
        }
    }

    fn jump(&mut self, inst: u16) {
        self.pc = Emulator::get_immediate_addr(inst);
    }

    // TODO: Add errors for invalid register

    fn set(&mut self, inst: u16) {
        let reg = Emulator::get_first_reg(inst);
        let val = Emulator::get_double_immediate_number(inst);

        self.regs[reg as usize] = val;
    }

    fn add(&mut self, inst: u16) {
        let reg = Emulator::get_first_reg(inst);
        let val = Emulator::get_double_immediate_number(inst);

        self.regs[reg as usize] += val;
    }

    fn set_index(&mut self, inst: u16) {
        let val = Emulator::get_immediate_addr(inst);

        self.index = val;
    }

    fn display(&mut self, inst: u16) {
        // TODO: Validate instruction
        let x = (self.regs[Emulator::get_first_reg(inst) as usize] % 64) as usize;
        let y = (self.regs[Emulator::get_second_reg(inst) as usize] % 32) as usize;
        let n = Emulator::get_immediate_number(inst);

        self.regs[0xf] = 0;

        for row in 0..n {
            let curr_y = y + row as usize;

            if curr_y >= DISPLAY_HEIGHT {
                break;
            }

            let sprite_byte = self.memory[(self.index + row as u16) as usize];

            for i in 0..8 {
                let curr_x = x + i;

                if curr_x >= DISPLAY_WIDTH {
                    break;
                }

                let bit = (sprite_byte >> (7 - i)) & 1;

                if bit == 1 && self.display[curr_y][curr_x] == 1 {
                    self.regs[0xf] = 1;
                }

                self.display[curr_y][curr_x] ^= bit;
            }
        }
    }

    fn subroutine_call(&mut self, inst: u16) {
        self.stack.push(self.pc);

        self.jump(inst);
    }

    fn subroutine_ret(&mut self) {
        if let Some(addr) = self.stack.pop() {
            self.pc = addr;
        }
    }

    fn skip(&mut self, inst: u16) {
        let op = Emulator::get_opcode(inst);
        let vx = self.regs[Emulator::get_first_reg(inst) as usize];
        let vy = self.regs[Emulator::get_second_reg(inst) as usize];
        let imm = Emulator::get_double_immediate_number(inst);

        match op {
            0x3 => {
                if vx == imm {
                    self.pc += 2;
                }
            }

            0x4 => {
                if vx != imm {
                    self.pc += 2;
                }
            }

            0x5 => {
                if vx == vy {
                    self.pc += 2;
                }
            }

            0x9 => {
                if vx != vy {
                    self.pc += 2;
                }
            }
            _ => {}
        };
    }
}
