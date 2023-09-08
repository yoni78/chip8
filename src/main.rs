mod emulator;
mod display;

use emulator::Emulator;
use display::{Display, GLFWDisplay};

fn main() {
    let display: Box<dyn Display> = Box::new(GLFWDisplay::new());
    let emulator = Emulator::new(display);
}
