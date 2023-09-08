mod emulator;
mod display;

use emulator::Emulator;
use display::{Display, OpenGLDisplay};

fn main() {
    let display: Box<dyn Display> = Box::new(OpenGLDisplay::new());
    let mut emulator = Emulator::new(display);

    let prog = [0x0];

    emulator.execute_program(&prog);
}
