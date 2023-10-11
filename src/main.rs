mod display;
mod emulator;

use display::{Display, PistonDisplay};
use emulator::Emulator;

fn main() {
    let mut chip_diplay = PistonDisplay::new(Emulator::new());

    let program = vec![0xd1, 0x11];

    chip_diplay.start(program);
}
