use motorino::shaders::ShaderProgram;
use super::specs::System;
use motorino::components::RawModel;
use super::specs::ReadStorage;
use super::specs::Read;
use motorino::camera::Camera;
use motorino::WindowSize;
use super::specs::Write;
use motorino::debugui::DebugInfo;
use motorino::resources::ResourceLoader;
use super::cgmath::{Deg, perspective};
use motorino::renderers::RenderConstants;
use super::gl;
use std::ffi::CStr;
use motorino::components::SkyboxFlag;
use motorino::environment::Fog;
use motorino::UpdateDeltaTime;
use super::cgmath::Matrix4;
use motorino::components::SkyboxTexture;


pub struct SkyboxRenderer {
    pub skybox_shader: ShaderProgram,
    pub rotation: f32,
    pub day_night_blend_factor: f32
}

impl<'a> System<'a> for SkyboxRenderer {

    type SystemData = (ReadStorage<'a, RawModel>,
                       ReadStorage<'a, SkyboxTexture>,
                       ReadStorage<'a, SkyboxFlag>,
                       Read<'a, Camera>,
                       Read<'a, Fog>,
                       Read<'a, WindowSize>,
                       Read<'a, UpdateDeltaTime>,
                       Write<'a, DebugInfo>);

    fn run(&mut self, (model, skybox_texture, skybox_flag, camera, fog, window_size, dt, mut debug_info): Self::SystemData) {
        self.rotation = self.rotation + (dt.0 * SkyboxRenderer::ROTATION_SPEED);
        self.prepare(&window_size);
        self.skybox_shader.start();
        self.bind_environment(&camera, &fog);

        use super::specs::Join;
        for (model, skybox_texture, _skybox_flag) in (&model, &skybox_texture, &skybox_flag).join() {
            self.bind_model(&model);
            self.bind_texture(&skybox_texture);
            debug_info.current_frame_draw_calls = debug_info.current_frame_draw_calls + 1;
            debug_info.current_frame_triangle_count = debug_info.current_frame_triangle_count + (model.vertex_count/3) as i32;
            unsafe {
                gl::DrawArrays(gl::TRIANGLES, 0, model.vertex_count as i32);
            }
            self.unbind_model();
        }
        self.skybox_shader.stop();
    }
}

impl SkyboxRenderer {

    const VERTEX_SHADER: &'static str = "shaders/skybox.vert";
    const FRAGMENT_SHADER: &'static str = "shaders/skybox.frag";
    const ROTATION_SPEED: f32 = 0.5;

    pub fn new(resource_loader: &ResourceLoader) -> SkyboxRenderer {
        let skybox_shader = ShaderProgram::from_shader_files(&resource_loader, SkyboxRenderer::VERTEX_SHADER, SkyboxRenderer::FRAGMENT_SHADER).unwrap();
        SkyboxRenderer {skybox_shader, rotation: 0.0, day_night_blend_factor: 0.5}
    }

    fn bind_environment(&self, camera: &Camera, fog: &Fog) {
        unsafe {
            let mut view_matrix = camera.get_view_matrix().clone();
            view_matrix.w.x = 0.0;
            view_matrix.w.y = 0.0;
            view_matrix.w.z = 0.0;
            view_matrix = view_matrix * Matrix4::from_angle_y(Deg(self.rotation));
            self.skybox_shader.set_mat4(c_str!("view_matrix"), &view_matrix);
            self.skybox_shader.set_vector3(c_str!("day_sky_colour"), &fog.day_colour);
            self.skybox_shader.set_vector3(c_str!("night_sky_colour"), &fog.night_colour);
            self.skybox_shader.set_float(c_str!("blend_factor"), self.day_night_blend_factor);
        }
    }

    fn bind_model(&self, raw_model: &RawModel) {
        unsafe {
            gl::BindVertexArray(raw_model.vao_id);
            gl::EnableVertexAttribArray(0); // positions
        }
    }

    fn unbind_model(&self) {
        unsafe {
            gl::DisableVertexAttribArray(0);
            gl::BindVertexArray(0);
        }
    }

    fn bind_texture(&self, skybox_texture: &SkyboxTexture) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, skybox_texture.day_texture.texture_id);

            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, skybox_texture.night_texture.texture_id);
        }
    }

    pub fn prepare(&self, window_size: &WindowSize) {
        self.skybox_shader.start();
        unsafe {
            //TODO we only need to run this once on load
            let projection_matrix = perspective(Deg(RenderConstants::FOV), window_size.width as f32 / window_size.height as f32, RenderConstants::NEAR_PLANE, RenderConstants::FAR_PLANE);
            self.skybox_shader.set_mat4(c_str!("projection_matrix"), &projection_matrix);
            self.skybox_shader.set_int(c_str!("day_cube_map"), 0);
            self.skybox_shader.set_int(c_str!("night_cube_map"), 1);

        }
        self.skybox_shader.stop();
    }
}