use piston_window::*;
use std::{thread, time::Duration};

use crate::emulator::Emulator;

const PIXEL_HEIGHT: usize = 10;
const PIXEL_WIDTH: usize = 10;
pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
const CLOCK_FREQ: f32 = 700.0;

pub trait Display {
    fn new(emulator: Emulator) -> Self;
    fn start(&mut self, program: Vec<u8>);
}

pub struct PistonDisplay {
    emulator: Emulator,
}

impl Display for PistonDisplay {
    fn new(emulator: Emulator) -> Self {
        Self { emulator }
    }

    fn start(&mut self, program: Vec<u8>) {
        let mut window: PistonWindow = WindowSettings::new("CHIP-8", [640, 320])
            .exit_on_esc(true)
            .resizable(false)
            .build()
            .unwrap();

        self.emulator.execute_program(&program);

        while let Some(e) = window.next() {
            self.emulator.execute_next_instruction();

            window.draw_2d(&e, |c, g, _device| {
                clear([0.0; 4], g);

                for i in 0..DISPLAY_HEIGHT {
                    for j in 0..DISPLAY_WIDTH {
                        if self.emulator.display[i][j] == 1 {
                            rectangle(
                                [1.0, 1.0, 1.0, 1.0],
                                [
                                    (j * PIXEL_WIDTH) as f64,
                                    (i * PIXEL_HEIGHT) as f64,
                                    PIXEL_WIDTH as f64,
                                    PIXEL_HEIGHT as f64,
                                ],
                                c.transform,
                                g,
                            );
                        }
                    }
                }
            });

            thread::sleep(Duration::from_secs_f32(1.0 / CLOCK_FREQ));
        }
    }
}
