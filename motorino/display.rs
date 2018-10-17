extern crate gl;
extern crate glfw;

use super::glfw::Action;
use super::glfw::{Context, Glfw, Window, WindowEvent};
use super::glfw::MouseButton;
use std::sync::mpsc::Receiver;


pub struct Display {
    pub should_close: bool,
    pub window: Window,
    glfw: Glfw
}

impl Display {

    pub fn create(width: u32, height: u32) -> (Display, Receiver<(f64, WindowEvent)>) {
        let (mut window, events, glfw) = Display::init(width, height);
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        (Display {
            should_close: false,
            window: window,
            glfw: glfw
        }, events)
    }

    fn init(width: u32, height: u32) -> (Window, Receiver<(f64, WindowEvent)>, Glfw) {
        // glfw: initialize and configure
        // ------------------------------
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        #[cfg(target_os = "macos")]
            glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        // glfw window creation
        // --------------------
        let (mut window, events) = glfw.create_window(width, height, "Motorino", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");
        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        return (window, events, glfw);
    }


    pub fn get_time(&self) -> f64 {
        return self.glfw.get_time();
    }

    #[allow(dead_code)]
    pub fn should_close(&self) -> bool {
        return self.should_close;
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    #[allow(dead_code)]
    pub fn set_should_close(&mut self, should_close: bool) {
        self.should_close = should_close;
    }

    pub fn poll_events(&mut self) {
        self.glfw.poll_events()
    }

    pub fn mouse_action(&self, mouse_button: MouseButton) -> Action {
        self.window.get_mouse_button(mouse_button)
    }

    pub fn get_cursor_position(&self) -> (f64, f64){
        self.window.get_cursor_pos()
    }
}
