use super::specs::{System, Write, Read};
use super::UpdateDeltaTime;
use super::camera::Camera;
use super::environment::{Light, Fog};
use super::imgui::{ImGui, ImGuiCond, FrameSize};
use super::imgui_opengl_renderer::Renderer as ImguiRenderer;
use super::glfw::{Action, Key};
use super::WindowSize;
use super::CursorPosition;
use super::renderers::RenderSettings;
use super::input::KeyEvents;
use super::input::MouseState;
use super::ringbuffer::RingBuffer;
use super::specs::ReadStorage;
use super::specs::WriteStorage;
use super::components::PlayerFlag;
use super::components::Transform;

pub struct DebugInfo {
    pub frame_times: RingBuffer<f32>,
    pub triangle_counts: RingBuffer<i32>,
    pub draw_calls: RingBuffer<i32>,
    pub current_frame_triangle_count: i32,
    pub current_frame_draw_calls: i32
}

impl Default for DebugInfo {
    fn default() -> Self {
        DebugInfo {
            frame_times: RingBuffer::<f32>::new(50),
            triangle_counts: RingBuffer::<i32>::new(50),
            draw_calls: RingBuffer::<i32>::new(50),
            current_frame_triangle_count: 0,
            current_frame_draw_calls: 0
        }
    }
}

pub struct DebugInfoResetter;

impl<'a> System<'a> for DebugInfoResetter {
    type SystemData = (Write<'a, DebugInfo>, Read<'a, UpdateDeltaTime>);

    fn run(&mut self, (mut debug_info, dt): <Self as System<'a>>::SystemData) {
        let last_triangle_count = debug_info.current_frame_triangle_count;
        let last_draw_call_count = debug_info.current_frame_draw_calls;
        debug_info.triangle_counts.push(last_triangle_count);
        debug_info.draw_calls.push(last_draw_call_count);
        debug_info.frame_times.push(dt.0);
        debug_info.current_frame_triangle_count = 0;
        debug_info.current_frame_draw_calls = 0;
    }
}

pub struct DebugUi {
    pub imgui: Option<ImGui>,
    pub imgui_renderer: Option<ImguiRenderer>
}

impl DebugUi {
    pub fn new<F>(load_fn: F) -> DebugUi
        where
            F: FnMut(&'static str) -> *const ::std::os::raw::c_void
    {
        let mut imgui = ImGui::init();
        let imgui_renderer = ImguiRenderer::new (&mut imgui, load_fn);
        let mut debug_ui = DebugUi {imgui: Some(imgui), imgui_renderer: Some(imgui_renderer)};
        debug_ui.setup_keyboard_input_keys();
        debug_ui
    }

    pub fn borrow_parts(&mut self) -> (&mut Option<ImGui>, &mut Option<ImguiRenderer>) {
        (&mut self.imgui, &mut self.imgui_renderer)
    }

    pub fn setup_keyboard_input_keys(&mut self) {
        use imgui::ImGuiKey;
        match &mut self.imgui {
            Some(imgui) => {
                imgui.set_imgui_key(ImGuiKey::Tab, 0);
                imgui.set_imgui_key(ImGuiKey::LeftArrow, 1);
                imgui.set_imgui_key(ImGuiKey::RightArrow, 2);
                imgui.set_imgui_key(ImGuiKey::UpArrow, 3);
                imgui.set_imgui_key(ImGuiKey::DownArrow, 4);
                imgui.set_imgui_key(ImGuiKey::PageUp, 5);
                imgui.set_imgui_key(ImGuiKey::PageDown, 6);
                imgui.set_imgui_key(ImGuiKey::Home, 7);
                imgui.set_imgui_key(ImGuiKey::End, 8);
                imgui.set_imgui_key(ImGuiKey::Delete, 9);
                imgui.set_imgui_key(ImGuiKey::Backspace, 10);
                imgui.set_imgui_key(ImGuiKey::Enter, 11);
                imgui.set_imgui_key(ImGuiKey::Escape, 12);
                imgui.set_imgui_key(ImGuiKey::A, 13);
                imgui.set_imgui_key(ImGuiKey::C, 14);
                imgui.set_imgui_key(ImGuiKey::V, 15);
                imgui.set_imgui_key(ImGuiKey::X, 16);
                imgui.set_imgui_key(ImGuiKey::Y, 17);
                imgui.set_imgui_key(ImGuiKey::Z, 18);
            },
            None => {}
        }

    }

    pub fn set_keyboard_input_state(&mut self, key_events: &KeyEvents) {
        match &mut self.imgui {
            Some(imgui) => {
                for key_event in key_events.iter() {
                    let pressed = key_event.action == Action::Press;
                    match key_event.source {
                        Key::Tab =>  imgui.set_key(0, pressed),
                        Key::Left =>  imgui.set_key(1, pressed),
                        Key::Right =>  imgui.set_key(2, pressed),
                        Key::Up =>  imgui.set_key(3, pressed),
                        Key::Down =>  imgui.set_key(4, pressed),
                        Key::PageUp =>  imgui.set_key(5, pressed),
                        Key::PageDown =>  imgui.set_key(6, pressed),
                        Key::Home =>  imgui.set_key(7, pressed),
                        Key::End =>  imgui.set_key(8, pressed),
                        Key::Delete =>  imgui.set_key(9, pressed),
                        Key::Backspace =>  imgui.set_key(10, pressed),
                        Key::Enter =>  imgui.set_key(11, pressed),
                        Key::Escape =>  imgui.set_key(12, pressed),
                        Key::A =>  imgui.set_key(13, pressed),
                        Key::C =>  imgui.set_key(14, pressed),
                        Key::V =>  imgui.set_key(15, pressed),
                        Key::X =>  imgui.set_key(16, pressed),
                        Key::Y =>  imgui.set_key(17, pressed),
                        Key::Z =>  imgui.set_key(18, pressed),
                        Key::LeftControl | Key::RightControl => {
                            imgui.set_key_ctrl(pressed)
                        }
                        Key::LeftShift | Key::RightShift =>  imgui.set_key_shift(pressed),
                        Key::LeftAlt | Key::RightAlt =>  imgui.set_key_alt(pressed),
                        Key::LeftSuper | Key::RightSuper =>  imgui.set_key_super(pressed),
                        _ => {}
                    }
                }
            },
            None => {}
        }
    }

    pub fn set_mouse_state(&mut self, cursor_position: &CursorPosition, mouse_state: &MouseState) {

        match &mut self.imgui {
            Some(imgui) => {
                imgui.set_mouse_pos(cursor_position.x, cursor_position.y);
                imgui.set_mouse_down([
                    mouse_state.button1,
                    mouse_state.button2,
                    mouse_state.button3,
                    false,
                    false
                ]);
            },
            None => {}
        }
    }
}

impl Default for DebugUi {
    fn default() -> Self {
        DebugUi{imgui: None, imgui_renderer: None}
    }
}

unsafe impl Sync for DebugUi {
}

pub struct DebugUiBuilder;

impl<'a> System<'a> for DebugUiBuilder {
    type SystemData = (Write<'a, Camera>,
                       Write<'a, Light>,
                       Write<'a, Fog>, 
                       Write<'a, DebugUi>,
                       Read<'a, KeyEvents>,
                       Read<'a, MouseState>,
                       Read<'a, UpdateDeltaTime>,
                       Read<'a, DebugInfo>,
                       Read<'a, WindowSize>,
                       Read<'a, CursorPosition>,
                       Read<'a, RenderSettings>,
                       ReadStorage<'a, PlayerFlag>,
                       WriteStorage<'a, Transform>);

    fn run(&mut self, (mut camera,
        mut light,
        mut fog,
        mut debug_ui,
        input_events,
        mouse_state,
        dt,
        debug_info,
        window_size,
        cursor_position,
        render_settings,
        player_flag,
        mut transform): Self::SystemData) {

        if !render_settings.debug_ui ||
            debug_ui.imgui.is_none() ||
            debug_ui.imgui_renderer.is_none() {
            return
        }
        debug_ui.set_keyboard_input_state(&input_events);
        debug_ui.set_mouse_state(&cursor_position, &mouse_state);

        let sum_frame_time:f32 = debug_info.frame_times.deque().iter().sum();

        let (imgui_opt, imgui_renderer_opt) = debug_ui.borrow_parts();
        let imgui = imgui_opt.as_mut().unwrap();
        let imgui_renderer = imgui_renderer_opt.as_ref().unwrap();

        let dt = dt.0;
        let ui = imgui.frame(FrameSize::new(window_size.width as f64, window_size.height as f64, 2.0), dt);
        ui.window(im_str!("Debug info"))
            .position((10.0, 10.0), ImGuiCond::FirstUseEver)
            .size((300.0, 300.0), ImGuiCond::FirstUseEver)
            .build(|| {

                let fps_times = debug_info.frame_times.deque().iter().map(|&x| 1.0/x).collect::<Vec<f32>>();
                ui.plot_lines(im_str!("Fps"), &fps_times).scale_min(0.0).scale_max(60.0).build();
                ui.text(im_str!("Triangles: {:.1}", debug_info.current_frame_triangle_count));
                ui.text(im_str!("Draw calls: {:.1}", debug_info.current_frame_draw_calls));
                ui.text(im_str!("Smoothed {:.1}", debug_info.frame_times.deque().len() as f32/sum_frame_time));
                ui.text(im_str!("Raw Mouse Position: ({:.1},{:.1})", cursor_position.x ,cursor_position.y));
                if ui.collapsing_header(im_str!("Player")).build() {
                    use super::specs::Join;
                    for (_player_flag, mut transform) in (&player_flag, &mut transform).join() {
                        ui.drag_float3(im_str!("Player position"), transform.position.as_mut()).build();
                    }
                }
                if ui.collapsing_header(im_str!("Camera")).build() {
                    ui.checkbox(im_str!("Follow player"), &mut camera.follow_player);
                    ui.drag_float3(im_str!("Camera position"), camera.position.as_mut()).build();
                    let mut yaw_pitch = [camera.yaw(), camera.pitch()];
                    if ui.drag_float2(im_str!("Yaw / Pitch"),&mut yaw_pitch).build() {
                        camera.set_yaw(yaw_pitch[0]);
                        camera.set_pitch(yaw_pitch[1]);
                    }
                }
                if ui.collapsing_header(im_str!("Light")).build() {
                    ui.drag_float3(im_str!("Light position"), light.position.as_mut()).build();
                    ui.color_picker(im_str!("Light Color"), light.colour.as_mut() as &mut [f32; 3]).build();
                }
                if ui.collapsing_header(im_str!("Fog")).build() {
                    ui.slider_float(im_str!("fog density"), &mut fog.density, 0.0, 1.0).build();
                    ui.slider_float(im_str!("fog gradient"), &mut fog.gradient, 0.0, 10.0).build();
                    ui.color_picker(im_str!("Light Color"), fog.colour.as_mut() as &mut [f32; 3]).build();
                }
            });
        imgui_renderer.render(ui);
    }
}


pub struct RenderSettingsController;

impl<'a> System<'a> for RenderSettingsController {
    type SystemData = (Read<'a, KeyEvents>,
                       Write<'a, RenderSettings>);

    fn run(&mut self, (key_events, mut render_settings): <Self as System<'a>>::SystemData) {
        for key_event in key_events.iter() {
            let key = &key_event.source;
            let action = &key_event.action;
            if key == &Key::Slash && (action == &Action::Press) {
                render_settings.debug_ui = !render_settings.debug_ui
            }
        }
    }
}
