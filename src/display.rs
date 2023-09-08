extern crate glfw;
use std::sync::mpsc::Receiver;

use glfw::{Action, Context, Key, Glfw, Window, WindowEvent};

pub trait Display {
    fn start(&mut self);
    fn clear(&mut self);
    fn toggle_pixel(&mut self, x: u8, y: u8);
}

pub struct GLFWDisplay {
    glfw: Glfw,
    window: Window,
    events: Receiver<(f64, WindowEvent)>
}

impl GLFWDisplay {
    pub fn new() -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (window, events) = glfw.create_window(640, 320, "CHIP-8 Emulator", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        Self {
            glfw,
            window,
            events
        }
    }
}

impl Display for GLFWDisplay {
    fn start(&mut self) {
        self.window.make_current();
        self.window.set_key_polling(true);
    
        while !self.window.should_close() {
            self.window.swap_buffers();
    
            self.glfw.poll_events();

            for (_, event) in glfw::flush_messages(&self.events) {
                println!("{:?}", event);
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        self.window.set_should_close(true)
                    },
                    _ => {},
                }
            }
        }
    }

    fn clear(&mut self) {
        todo!()
    }

    fn toggle_pixel(&mut self, x: u8, y: u8) {
        todo!()
    }
}