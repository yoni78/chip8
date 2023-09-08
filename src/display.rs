extern crate glfw;
use std::sync::mpsc::Receiver;

use glfw::{Action, Context, Key, Glfw, Window, WindowEvent};

pub trait Display {
    fn start(&mut self);
    fn clear(&mut self);
    fn toggle_pixel(&mut self, x: u8, y: u8);
}

pub struct OpenGLDisplay {
    glfw: Glfw,
    window: Window,
    events: Receiver<(f64, WindowEvent)>
}

impl OpenGLDisplay {
    pub fn new() -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, events) = glfw.create_window(640, 320, "CHIP-8 Emulator", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        gl::load_with(|s| window.get_proc_address(s) as *const std::os::raw::c_void);

        Self {
            glfw,
            window,
            events
        }
    }
}

impl Display for OpenGLDisplay {
    fn start(&mut self) {
        self.window.make_current();
        self.window.set_key_polling(true);

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.5, 1.0);
        }
    
        while !self.window.should_close() {
            
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);    
            }

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