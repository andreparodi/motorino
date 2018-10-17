use super::cgmath::Vector3;
use super::specs::VecStorage;
use super::specs::Component;
use super::specs::NullStorage;
use super::terrain::Heights;
use super::models::Loader;
use super::gl::types::GLuint;


#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Velocity {
    pub run_velocity: f32,
    pub turn_velocity: f32,
    pub upwards_velocity: f32
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PlayerFlag;

impl Component for PlayerFlag {
    type Storage = NullStorage<Self>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GridPosition {
    pub x: i32,
    pub z: i32
}

impl Component for GridPosition {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Copy, Debug)]
pub struct TerrainTexturePack {
    pub background_texture: TerrainTexture,
    pub r_texture: TerrainTexture,
    pub g_texture: TerrainTexture,
    pub b_texture: TerrainTexture,
    pub blend_map_texture: TerrainTexture}

impl Component for TerrainTexturePack {
    type Storage = VecStorage<Self>;
}

impl TerrainTexturePack {
    pub fn new(loader: &mut Loader, background_texture: &str, r_texture: &str, g_texture: &str, b_texture: &str, blend_map_texture: &str) -> TerrainTexturePack {
        let background_texture = loader.load_terrain_texture(background_texture);
        let r_texture = loader.load_terrain_texture(r_texture);
        let g_texture = loader.load_terrain_texture(g_texture);
        let b_texture = loader.load_terrain_texture(b_texture);
        let blend_map_texture = loader.load_terrain_texture(blend_map_texture);
        TerrainTexturePack { background_texture, r_texture, g_texture, b_texture, blend_map_texture }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TerrainTexture {
    pub texture_id: GLuint
}

#[derive(Clone, Copy, Debug)]
pub struct SimpleTexture {
    pub texture_id: GLuint,
    pub reflectivity: f32,
    pub shine_damper: f32}

impl Component for SimpleTexture {
    type Storage = VecStorage<Self>;
}

impl Default for SimpleTexture {
    fn default() -> SimpleTexture {
        SimpleTexture {
            texture_id: 0,
            reflectivity: 0.0,
            shine_damper: 0.0
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RawModel {
    pub vao_id: GLuint,
    pub vertex_count: usize
}

impl Component for RawModel {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Copy)]
pub struct TerrainPhysics {
    pub heights: Heights,
    pub x: f32,
    pub z: f32
}

impl Component for TerrainPhysics {
    type Storage = VecStorage<Self>;
}
