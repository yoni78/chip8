mod emulator;

use emulator::Emulator;

fn main() {
    let mut emulator = Emulator::new();

    let prog = [0x0];

    emulator.execute_program(&prog);
}
