use super::cgmath::Vector2;
use super::specs::{Write, System};
use super::glfw::{Action, MouseButton, Key};
use super::glfw;
use super::gl;
use std::sync::mpsc::Receiver;
use super::glfw::WindowEvent;


#[derive(Clone, Copy, Debug, Default)]
pub struct CursorPosition {
    pub x: f32,
    pub y: f32
}

#[derive(Default)]
pub struct MouseState {
    pub button1: bool,
    pub button2: bool,
    pub button3: bool,
    pub button4: bool,
    pub button5: bool
}
pub struct InputEvent<T> {
    pub source: T,
    pub action: Action
}

pub type KeyEvents = Vec<InputEvent<Key>>;
pub type MouseEvents = Vec<InputEvent<MouseButton>>;


pub struct WindowEventHandler {
    pub drag_start: Option<Vector2<f32>>,
    pub event_receiver: Receiver<(f64, WindowEvent)>
}

impl WindowEventHandler {

    pub fn new(event_receiver: Receiver<(f64, WindowEvent)>) -> WindowEventHandler {
        WindowEventHandler {drag_start: None, event_receiver}
    }
}

impl<'a> System<'a> for WindowEventHandler {

    type SystemData = (Write<'a, KeyEvents>,
                       Write<'a, MouseEvents>);

    fn run(&mut self, (mut key_events, mut mouse_events): Self::SystemData) {
        key_events.clear();
        mouse_events.clear();
        for (_, event) in glfw::flush_messages(&self.event_receiver) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions; note that width and
                    // height will be significantly larger than specified on retina displays.
                    unsafe { gl::Viewport(0, 0, width, height) }
                }
                glfw::WindowEvent::Key(source, _, action, _) => {
                    key_events.push(InputEvent{source, action});
                }
                glfw::WindowEvent::MouseButton(source, action, _) => {
                    mouse_events.push(InputEvent{source, action});
                }
                //glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => self.set_should_close(true),
                _ => {}
            }
        }
    }
}


