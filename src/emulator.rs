use std::{thread, time::Duration};
use piston_window::*;

const MEM_SIZE: usize = 4 * 1024;
const CLOCK_FREQ: f32 = 700.0;
const PROGRAM_LOC: usize = 0x200;
const PIXEL_HEIGHT: usize = 10;
const PIXEL_WIDTH: usize = 10;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const REGS_COUNT: usize = 16;

pub struct Emulator {
    memory: Vec<u8>,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    pc: u16,
    index: u16,
    regs: [u8; REGS_COUNT],
    display: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
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

        let mut window: PistonWindow = 
            WindowSettings::new("CHIP-8", [640, 320])
            .exit_on_esc(true).build().unwrap();

        while let Some(e) = window.next() {
            let inst = self.fetch();
            self.execute(inst);
            
            window.draw_2d(&e, |c, g, _device| {
                clear([0.0; 4], g);
                
                for i in 0..DISPLAY_HEIGHT {
                    for j in 0..DISPLAY_WIDTH {
                        if self.display[i][j] == 1 {
                            rectangle([1.0, 1.0, 1.0, 1.0],
                                [(j * PIXEL_WIDTH) as f64, (i * PIXEL_HEIGHT) as f64, PIXEL_WIDTH as f64, PIXEL_HEIGHT as f64],
                                c.transform, g);
                        }
                    }
                }
                
            });

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
            0xd => self.display(inst),
            
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
        self.display[15][32] = 1;
    }
 
}
