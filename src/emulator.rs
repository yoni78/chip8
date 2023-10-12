use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

use crate::display::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

const MEM_SIZE: usize = 4096;
const PROGRAM_LOC: usize = 0x200;

const REGS_COUNT: usize = 16;
const FLAG_REG: usize = 0xf;

const FONT_START: usize = 0x50;
const FONT: &[u8] = &[
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

const TIMER_FREQ: f32 = 60.0;

pub struct Emulator {
    memory: [u8; MEM_SIZE],
    stack: Vec<u16>,
    delay_timer: Arc<(Mutex<u8>, Condvar)>,
    sound_timer: Arc<(Mutex<u8>, Condvar)>,
    pc: u16,
    index: u16,
    regs: [u8; REGS_COUNT],
    pub display: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    pub key_pressed: Option<u8>,
}

impl Emulator {
    pub fn new() -> Self {
        let mut memory = [0; MEM_SIZE];

        for i in 0..FONT.len() {
            memory[FONT_START + i] = FONT[i];
        }

        let delay_timer = Arc::new((Mutex::new(0), Condvar::new()));
        let sound_timer = Arc::new((Mutex::new(0), Condvar::new()));

        let delay_copy = delay_timer.clone();
        let sound_copy = delay_timer.clone();

        thread::spawn(move || timer_worker(delay_copy));
        thread::spawn(move || timer_worker(sound_copy));

        Self {
            memory,
            stack: Vec::new(),
            delay_timer,
            sound_timer,
            pc: 0,
            index: 0,
            regs: [0; REGS_COUNT],
            display: [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            key_pressed: Option::None,
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
        let imm = Emulator::get_double_immediate_number(inst);

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
            0x8 => self.logical_arithmetic_op(inst),
            0xa => self.set_index(inst),
            0xb => self.jump_with_offset(inst),
            0xc => self.random(inst),
            0xd => self.display(inst),
            0xe => self.skip_if_key(inst),
            0xf => match imm {
                0x07 => self.read_delay_timer(inst),
                0x15 => self.set_delay_timer(inst),
                0x18 => self.set_sound_timer(inst),
                0x1e => self.add_to_index(inst),
                0x29 => self.font_character(inst),
                0x33 => self.decimal_conversion(inst),
                0x55 => self.store_to_memory(inst),
                0x65 => self.load_from_memory(inst),
                0xa => self.get_key(inst),
                _ => {}
            },

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

        self.regs[FLAG_REG] = 0;

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
                    self.regs[FLAG_REG] = 1;
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

    // TODO: Add configuration for legacy shift operation

    fn logical_arithmetic_op(&mut self, inst: u16) {
        let imm = Emulator::get_immediate_number(inst);

        let vx_num = Emulator::get_first_reg(inst) as usize;
        let vy_num = Emulator::get_first_reg(inst) as usize;

        let vx = self.regs[vx_num];
        let vy = self.regs[vy_num];

        match imm {
            0x0 => {
                self.regs[vx_num] = vy;
            }

            0x1 => {
                self.regs[vx_num] = vx | vy;
            }

            0x2 => {
                self.regs[vx_num] = vx & vy;
            }

            0x3 => {
                self.regs[vx_num] = vx ^ vy;
            }

            0x4 => match vx.checked_add(vy) {
                Some(val) => self.regs[vx_num] = val,
                None => {
                    self.regs[vx_num] = vx + vy;
                    self.regs[FLAG_REG] = 1;
                }
            },

            0x5 => {
                self.regs[FLAG_REG] = if vx > vy { 1 } else { 0 };

                self.regs[vx_num] = vx - vy;
            }

            0x6 => {
                self.regs[FLAG_REG] = vx & 1;
                self.regs[vx_num] = vx >> 1;
            }

            0x7 => {
                self.regs[FLAG_REG] = if vy > vx { 1 } else { 0 };

                self.regs[vx_num] = vy - vx;
            }

            0xe => {
                self.regs[FLAG_REG] = vx & 0x80;
                self.regs[vx_num] = vx << 1;
            }

            _ => {}
        };
    }

    // TOOD: Add config for alt behavior

    fn jump_with_offset(&mut self, inst: u16) {
        let addr = Emulator::get_immediate_addr(inst);

        self.pc = addr + self.regs[0] as u16;
    }

    fn random(&mut self, inst: u16) {
        let imm = Emulator::get_double_immediate_number(inst);
        let rand_num: u8 = rand::random();
        let reg = Emulator::get_first_reg(inst) as usize;

        self.regs[reg] = rand_num & imm;
    }

    fn add_to_index(&mut self, inst: u16) {
        let reg = Emulator::get_first_reg(inst) as usize;

        self.index += self.regs[reg] as u16;

        if self.index > 0xfff {
            self.regs[FLAG_REG] = 1;
        }
    }

    fn font_character(&mut self, inst: u16) {
        let reg = Emulator::get_first_reg(inst) as usize;

        self.index = FONT_START as u16 + self.regs[reg] as u16;
    }

    fn decimal_conversion(&mut self, inst: u16) {
        let reg = Emulator::get_first_reg(inst) as usize;
        let val = self.regs[reg];

        self.memory[self.index as usize] = val / 100;
        self.memory[self.index as usize + 1] = (val / 10) % 10;
        self.memory[self.index as usize + 2] = val % 10;
    }

    // TODO: Add legacy index increment behavior config

    fn store_to_memory(&mut self, inst: u16) {
        let max_reg = Emulator::get_first_reg(inst);

        for i in 0..=max_reg {
            let loc = (self.index + i as u16) as usize;

            self.memory[loc] = self.regs[i as usize];
        }
    }

    fn load_from_memory(&mut self, inst: u16) {
        let max_reg = Emulator::get_first_reg(inst);

        for i in 0..=max_reg {
            let loc = (self.index + i as u16) as usize;

            self.regs[i as usize] = self.memory[loc];
        }
    }

    fn get_key(&mut self, inst: u16) {
        if let Some(key) = self.key_pressed {
            let reg = Emulator::get_first_reg(inst) as usize;

            self.regs[reg] = key;
        } else {
            self.pc -= 2
        }
    }

    fn skip_if_key(&mut self, inst: u16) {
        let imm = Emulator::get_double_immediate_number(inst);
        let vx = self.regs[Emulator::get_first_reg(inst) as usize];

        match imm {
            0x9e => {
                if let Some(key) = self.key_pressed {
                    if key == vx {
                        self.pc += 2;
                    }
                }
            }

            0xa1 => {
                if let Some(key) = self.key_pressed {
                    if key == vx {
                        return;
                    }
                }

                self.pc += 2;
            }

            _ => {}
        }
    }

    fn read_delay_timer(&mut self, inst: u16) {
        let (lock, _) = &*self.delay_timer;
        let timer = lock.lock().unwrap();

        self.regs[Emulator::get_first_reg(inst) as usize] = *timer;
    }

    fn set_delay_timer(&mut self, inst: u16) {
        let vx = self.regs[Emulator::get_first_reg(inst) as usize];

        let (lock, cvar) = &*self.delay_timer;
        let mut timer = lock.lock().unwrap();

        *timer = vx;

        cvar.notify_one();
    }

    fn set_sound_timer(&mut self, inst: u16) {
        let vx = self.regs[Emulator::get_first_reg(inst) as usize];

        let (lock, cvar) = &*self.sound_timer;
        let mut timer = lock.lock().unwrap();

        *timer = vx;

        cvar.notify_one();
    }
}

fn timer_worker(timer_data: Arc<(Mutex<u8>, Condvar)>) {
    let (lock, cvar) = &*timer_data;

    loop {
        thread::sleep(Duration::from_secs_f32(1.0 / TIMER_FREQ));

        let mut timer = lock.lock().unwrap();

        while *timer <= 0 {
            timer = cvar.wait(timer).unwrap();
        }

        *timer -= 1;
    }
}
