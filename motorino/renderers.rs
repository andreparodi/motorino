#![macro_use]

use std::ffi::CStr;
use std::ptr;
use super::camera::Camera;
use super::cgmath::{Deg, Matrix4, perspective, Vector3};
use super::components::{GridPosition, RawModel, SimpleTexture, Transform};
use super::environment::{Fog, Light};
use super::gl;
use super::resources::ResourceLoader;
use super::shaders::ShaderProgram;
use super::specs::{Read, ReadStorage, System};
use super::WindowSize;
use super::components::TerrainTexturePack;
use super::specs::Write;
use super::debugui::DebugInfo;

pub struct RenderConstants {
}

impl RenderConstants {
    const FOV: f32 = 45.0;
    const NEAR_PLANE: f32 = 0.1;
    const FAR_PLANE: f32 = 200.0;
}


#[derive(Clone, Copy, Debug, Default)]
pub struct RenderSettings {
    pub debug_ui: bool,
    pub wireframes: bool
}


pub struct EntityRenderer {
    pub entity_shader: ShaderProgram
}


impl<'a> System<'a> for EntityRenderer {

    type SystemData = (ReadStorage<'a, Transform>,
                       ReadStorage<'a, SimpleTexture>,
                       ReadStorage<'a, RawModel>,
                       Read<'a, Camera>,
                       Read<'a, Fog>,
                       Read<'a, Light>,
                       Read<'a, WindowSize>,
                       Write<'a, DebugInfo>);

    fn run(&mut self, (transform, simple_texture, model, camera, fog, light, window_size, mut debug_info): Self::SystemData) {
        self.prepare(&window_size);
        self.entity_shader.start();
        self.bind_environment(&camera, &light, &fog);

        use super::specs::Join;
        for (transform, model, texture) in (&transform, &model, &simple_texture).join() {
            self.bind_model(&model, &texture);
            self.bind_entity(&transform);
            debug_info.current_frame_draw_calls = debug_info.current_frame_draw_calls + 1;
            debug_info.current_frame_triangle_count = debug_info.current_frame_triangle_count + (model.vertex_count/3) as i32;
            unsafe {
                gl::DrawElements(gl::TRIANGLES, model.vertex_count as i32, gl::UNSIGNED_INT, ptr::null());
            }
            self.unbind_model();
        }
        self.entity_shader.stop();
    }
}

impl EntityRenderer {

    const VERTEX_SHADER: &'static str = "shaders/default.vert";
    const FRAGMENT_SHADER: &'static str = "shaders/default.frag";

    pub fn new(resource_loader: &ResourceLoader) -> EntityRenderer {
        let entity_shader = ShaderProgram::from_shader_files(&resource_loader, EntityRenderer::VERTEX_SHADER, EntityRenderer::FRAGMENT_SHADER).unwrap();
        EntityRenderer {entity_shader}
    }

    pub fn prepare(&self, window_size: &WindowSize) {
        self.entity_shader.start();
        unsafe {
            let projection_matrix = perspective(Deg(RenderConstants::FOV), window_size.width as f32 / window_size.height as f32, RenderConstants::NEAR_PLANE, RenderConstants::FAR_PLANE);
            self.entity_shader.set_mat4(c_str!("projection_matrix"), &projection_matrix);
        }
        self.entity_shader.stop();
    }

    fn bind_environment(&self, camera: &Camera, light: &Light, fog: &Fog) {
        unsafe {
            self.entity_shader.set_mat4(c_str!("view_matrix"), &camera.get_view_matrix());
            self.entity_shader.set_vector3(c_str!("light_position"), &light.position);
            self.entity_shader.set_vector3(c_str!("light_colour"), &light.colour);
            self.entity_shader.set_vector3(c_str!("sky_colour"), &fog.colour);
            self.entity_shader.set_float(c_str!("fog_density"), fog.density);
            self.entity_shader.set_float(c_str!("fog_gradient"), fog.gradient);
        }
    }

    fn bind_entity(&self, transform: &Transform) {
        unsafe {
            let t = Matrix4::from_translation(transform.position)
                * Matrix4::from_nonuniform_scale(transform.scale.x, transform.scale.y, transform.scale.z)
                * Matrix4::from_angle_x(Deg(transform.rotation.x))
                * Matrix4::from_angle_y(Deg(transform.rotation.y))
                * Matrix4::from_angle_z(Deg(transform.rotation.z));
            self.entity_shader.set_mat4(c_str!("transformation_matrix"), &t);
        }
    }

    fn bind_model(&self, model: &RawModel, texture: &SimpleTexture) {
        unsafe {
            gl::BindVertexArray(model.vao_id);
            gl::EnableVertexAttribArray(0); // positions
            gl::EnableVertexAttribArray(1); // texture coords
            gl::EnableVertexAttribArray(2); // normals
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture.texture_id);

            // set uniforms
            self.entity_shader.set_float(c_str!("reflectivity"), texture.reflectivity);
            self.entity_shader.set_float(c_str!("shine_damper"), texture.shine_damper);
        }
    }

    fn unbind_model(&self) {
        unsafe {
            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
            gl::DisableVertexAttribArray(2);
            gl::BindVertexArray(0);
        }
    }

}

pub struct ClearScreenRenderer;

impl<'a> System<'a> for ClearScreenRenderer {
    type SystemData = ();
    fn run(&mut self, (): Self::SystemData) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT|gl::DEPTH_BUFFER_BIT);
        }
    }
}

pub struct TerrainRenderer {
    pub terrain_shader: ShaderProgram,
}

impl<'a> System<'a> for TerrainRenderer {

    type SystemData = (ReadStorage<'a, TerrainTexturePack>,
                       ReadStorage<'a, RawModel>,
                       ReadStorage<'a, GridPosition>,
                       Read<'a, Camera>,
                       Read<'a, Fog>,
                       Read<'a, Light>,
                       Read<'a, WindowSize>,
                       Write<'a, DebugInfo>);

    fn run(&mut self, (terrain_texture_pack, model, grid_position, camera, fog, light, window_size, mut debug_info): Self::SystemData) {
        self.prepare(&window_size);
        self.terrain_shader.start();
        self.bind_environment(&camera, &light, &fog);

        use super::specs::Join;
        for (model, terrain_texture_pack, grid_position) in (&model, &terrain_texture_pack, &grid_position).join() {
            self.bind_model(&model);
            self.bind_terrain(&terrain_texture_pack, &grid_position);
            debug_info.current_frame_draw_calls = debug_info.current_frame_draw_calls + 1;
            debug_info.current_frame_triangle_count = debug_info.current_frame_triangle_count + (model.vertex_count/3) as i32;
            unsafe {
                gl::DrawElements(gl::TRIANGLES, model.vertex_count as i32, gl::UNSIGNED_INT, ptr::null());
            }
            self.unbind_model();
        }
        self.terrain_shader.stop();
    }
}

impl TerrainRenderer {

    const VERTEX_SHADER: &'static str = "shaders/terrain.vert";
    const FRAGMENT_SHADER: &'static str = "shaders/terrain.frag";

    pub fn new(resource_loader: &ResourceLoader) -> TerrainRenderer {
        let terrain_shader = ShaderProgram::from_shader_files(&resource_loader, TerrainRenderer::VERTEX_SHADER, TerrainRenderer::FRAGMENT_SHADER).unwrap();
        TerrainRenderer {terrain_shader}
    }

    fn bind_environment(&self, camera: &Camera, light: &Light, fog: &Fog) {
        unsafe {
            self.terrain_shader.set_mat4(c_str!("view_matrix"), &camera.get_view_matrix());
            self.terrain_shader.set_vector3(c_str!("light_position"), &light.position);
            self.terrain_shader.set_vector3(c_str!("light_colour"), &light.colour);
            self.terrain_shader.set_vector3(c_str!("sky_colour"), &fog.colour);
            self.terrain_shader.set_float(c_str!("fog_density"), fog.density);
            self.terrain_shader.set_float(c_str!("fog_gradient"), fog.gradient);
        }
    }

    fn bind_model(&self, raw_model: &RawModel) {
        unsafe {
            gl::BindVertexArray(raw_model.vao_id);
            gl::EnableVertexAttribArray(0); // positions
            gl::EnableVertexAttribArray(1); // texture coords
            gl::EnableVertexAttribArray(2); // normals

            // set uniforms
            self.terrain_shader.set_float(c_str!("reflectivity"), 0.0);
            self.terrain_shader.set_float(c_str!("shine_damper"), 1.0);
        }
    }

    fn unbind_model(&self) {
        unsafe {
            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
            gl::DisableVertexAttribArray(2);
            gl::BindVertexArray(0);
        }
    }

    fn bind_terrain(&self, terrain_texture_pack: &TerrainTexturePack, grid_position: &GridPosition) {
        unsafe {
            self.bind_textures(&terrain_texture_pack);
            let transformation = Matrix4::from_translation(Vector3 {x: grid_position.x as f32, y: 0.0, z: grid_position.z as f32});
            self.terrain_shader.set_mat4(c_str!("transformation_matrix"), &transformation);
        }
    }

    fn bind_textures(&self, terrain_texture_pack: &TerrainTexturePack) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, terrain_texture_pack.background_texture.texture_id);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, terrain_texture_pack.r_texture.texture_id);
            gl::ActiveTexture(gl::TEXTURE2);
            gl::BindTexture(gl::TEXTURE_2D, terrain_texture_pack.g_texture.texture_id);
            gl::ActiveTexture(gl::TEXTURE3);
            gl::BindTexture(gl::TEXTURE_2D, terrain_texture_pack.b_texture.texture_id);
            gl::ActiveTexture(gl::TEXTURE4);
            gl::BindTexture(gl::TEXTURE_2D, terrain_texture_pack.blend_map_texture.texture_id);
        }
    }

    pub fn prepare(&self, window_size: &WindowSize) {
        self.terrain_shader.start();
        unsafe {
            //TODO we only need to run this once on load
            let projection_matrix = perspective(Deg(RenderConstants::FOV), window_size.width as f32 / window_size.height as f32, RenderConstants::NEAR_PLANE, RenderConstants::FAR_PLANE);
            self.terrain_shader.set_mat4(c_str!("projection_matrix"), &projection_matrix);
            self.terrain_shader.set_int(c_str!("background_sampler"), 0);
            self.terrain_shader.set_int(c_str!("r_sampler"), 1);
            self.terrain_shader.set_int(c_str!("g_sampler"), 2);
            self.terrain_shader.set_int(c_str!("b_sampler"), 3);
            self.terrain_shader.set_int(c_str!("blend_map_sampler"), 4);
        }
        self.terrain_shader.stop();
    }
}