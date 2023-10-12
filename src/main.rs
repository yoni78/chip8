mod display;
mod emulator;

use display::{Display, PistonDisplay};
use emulator::Emulator;
use std::fs;

fn main() {
    let mut chip_diplay = PistonDisplay::new(Emulator::new());

    let program = fs::read("roms/test_opcode.ch8").unwrap();

    chip_diplay.start(program);
}
