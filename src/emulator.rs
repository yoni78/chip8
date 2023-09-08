use crate::display::Display;
use std::{thread, time::Duration};

const MEM_SIZE: usize = 4 * 1024;
const CLOCK_FREQ: f32 = 700.0;
const PROGRAM_LOC: usize = 0x200;

pub struct Emulator {
    memory: Vec<u8>,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    pc: u16,
    index: u16,
    regs: [u8; 16],
    display: Box<dyn Display>
}

impl Emulator {
    pub fn new(display: Box<dyn Display>) -> Self {
        Self {
            memory: Vec::with_capacity(MEM_SIZE),
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            pc: 0,
            index: 0,
            regs: [0; 16],
            display,
        }
    }

    pub fn execute_program(&mut self, program_data: &[u8]) {
        self.load_program(program_data);
        self.pc = PROGRAM_LOC as u16;

        self.display.start();

        loop {
            let inst = self.fetch();
            self.execute(inst);

            thread::sleep(Duration::from_secs_f32(1.0 / CLOCK_FREQ));
        }
    }

    fn load_program(&mut self, program_data: &[u8]) {
        for (i, byte) in program_data.iter().enumerate() {
            self.memory[PROGRAM_LOC + i] = *byte;
        }
    }

    fn fetch(&mut self) -> u16 {
        // TODO: Little or big endian
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
            0x0 => {
                match inst {
                    0x00e0 => self.clear_screen(),

                    _ => {}
                }
            }

            0x1 => self.jump(inst),
            0x6 => self.set(inst),
            0x7 => self.add(inst),
            0xa => self.set_index(inst),
            
            // TODO: Invalid instruction error
            _ => {}
        }
    }
    
    fn get_opcode(inst: u16) -> u16 {
        (inst & 0xf000) >> 12
    }

    fn get_first_reg(inst: u16) -> u16 {
        (inst & 0x0f00) >> 8
    }

    fn get_second_reg(inst: u16) -> u16 {
        (inst & 0x00f0) >> 4
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
        self.display.clear();
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
 
}