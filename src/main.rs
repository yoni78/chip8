mod display;
mod emulator;

use display::{Display, PistonDisplay};
use emulator::Emulator;
use std::fs;

fn main() {
    let mut chip_diplay = PistonDisplay::new(Emulator::new());

    let program = fs::read("sample_programs/ibm_logo.ch8").unwrap();

    chip_diplay.start(program);
}
