mod emulator;

use emulator::Emulator;

fn main() {
    let mut emulator = Emulator::new();

    let prog = [0xd1, 0x11];

    emulator.execute_program(&prog);
}
