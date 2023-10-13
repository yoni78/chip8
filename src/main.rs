mod display;
mod emulator;

use display::{Display, PistonDisplay};
use emulator::Emulator;
use std::{env, fs};

const ARG_ERROR: &str = "The first argument should be a path to a valid CHIP-8 ROM.";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("{}", ARG_ERROR);
        return;
    }

    match fs::read(&args[1]) {
        Ok(program) => PistonDisplay::new(Emulator::new()).start(program),

        Err(_) => println!("{}", ARG_ERROR),
    }
}
