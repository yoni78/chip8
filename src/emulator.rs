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
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    va: u8,
    vb: u8,
    vc: u8,
    vd: u8,
    ve: u8,
    vf: u8,
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
            v0: 0,
            v1: 0,
            v2: 0,
            v3: 0,
            v4: 0,
            v5: 0,
            v6: 0,
            v7: 0,
            v8: 0,
            v9: 0,
            va: 0,
            vb: 0,
            vc: 0,
            vd: 0,
            ve: 0,
            vf: 0,
            display,
        }
    }

    pub fn execute_program(&mut self, program_data: &[u8]) {
        self.load_program(program_data);

        loop {
            let inst = self.fetch();

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

    fn decode(&mut self, inst: u16) {

    }
}