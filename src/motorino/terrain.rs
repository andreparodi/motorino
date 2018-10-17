
use super::cgmath::prelude::*;
use super::cgmath::Vector3;
use super::models::Loader;
use super::components::RawModel;
use super::image::DynamicImage;
use super::image::GenericImageView;
use super::cgmath::Vector2;
use super::specs::ReadStorage;
use super::components::GridPosition;
use super::components::TerrainPhysics;

pub type Heights = [[f32; Terrain::VERTEX_COUNT as usize]; Terrain::VERTEX_COUNT as usize];

#[derive(Copy, Clone)]
pub struct Terrain {
    pub x: f32,
    pub z: f32,
    pub raw_model: RawModel,
    pub heights: Heights,
}

impl Terrain {
    pub const SIZE: u32 = 800;
    const VERTEX_COUNT: u32 = 256;
    const MAX_HEIGHT: f32 = 40.0;

    pub fn new(loader: &mut Loader, gridx: u32, gridz: u32, height_map: &DynamicImage) -> Terrain {
        let (raw_model, heights) = Terrain::generate_terrain(loader, &height_map);
        Terrain { x: (gridx * Terrain::SIZE) as f32,
            z: (gridz * Terrain::SIZE) as f32,
            raw_model,
            heights
        }
    }

    pub fn generate_terrain(loader: &mut Loader, height_map: &DynamicImage) -> (RawModel, Heights) {
        assert!(height_map.height() >= Terrain::VERTEX_COUNT);

        let mut vertices = [0.0; (Terrain::VERTEX_COUNT * Terrain::VERTEX_COUNT * 3) as usize];
        let mut normals = [0.0; (Terrain::VERTEX_COUNT * Terrain::VERTEX_COUNT * 3) as usize];
        let mut texture_coords = [0.0; (Terrain::VERTEX_COUNT * Terrain::VERTEX_COUNT * 2) as usize];

        let mut indices = [0 as u32; (6 * (Terrain::VERTEX_COUNT - 1) * (Terrain::VERTEX_COUNT - 1)) as usize];
        let mut vertex_pointer = 0;
        let mut heights: Heights = [[0.0; Terrain::VERTEX_COUNT as usize]; Terrain::VERTEX_COUNT as usize];

        for i in 0..Terrain::VERTEX_COUNT {
            for j in 0..Terrain::VERTEX_COUNT {
                vertices[(vertex_pointer * 3)] = j as f32 / (Terrain::VERTEX_COUNT - 1) as f32 * Terrain::SIZE as f32;
                let height = Terrain::get_height(&height_map, i, j);
                heights[j as usize][i as usize] = height;
                vertices[(vertex_pointer * 3) + 1] = height;
                vertices[(vertex_pointer * 3) + 2] = i as f32 / (Terrain::VERTEX_COUNT - 1) as f32 * Terrain::SIZE as f32;

                let normal = Terrain::calculate_normal(height_map, i, j);
                normals[(vertex_pointer * 3)] = normal.x;
                normals[(vertex_pointer * 3) + 1] = normal.y;
                normals[(vertex_pointer * 3) + 2] = normal.z;
                texture_coords[(vertex_pointer * 2)] = j as f32 / (Terrain::VERTEX_COUNT - 1) as f32;
                texture_coords[(vertex_pointer * 2) + 1] = i as f32 / (Terrain::VERTEX_COUNT - 1) as f32;
                vertex_pointer += 1;
            }
        }

        let mut pointer = 0;
        for gz in 0..Terrain::VERTEX_COUNT - 1 {
            for gx in 0..Terrain::VERTEX_COUNT - 1 {
                let top_left = (gz * Terrain::VERTEX_COUNT) + gx;
                let top_right = top_left + 1;
                let bottom_left = ((gz + 1) * Terrain::VERTEX_COUNT) + gx;
                let bottom_right = bottom_left + 1;
                indices[pointer] = top_left as u32;
                indices[pointer + 1] = bottom_left as u32;
                indices[pointer + 2] = top_right as u32;
                indices[pointer + 3] = top_right as u32;
                indices[pointer + 4] = bottom_left as u32;
                indices[pointer + 5] = bottom_right as u32;
                pointer += 6;
            }
        }
        (loader.load_to_vao(&vertices, &texture_coords, &normals, &indices), heights)
    }

    fn get_height(height_map: &DynamicImage, x: u32, z: u32) -> f32 {
        if x >= height_map.height() || z >= height_map.height() {
            return 0.0;
        }
        let rgba = height_map.get_pixel(x, z);
        let r = rgba.data[0] as f32;
        let g = rgba.data[1] as f32;
        let b = rgba.data[2] as f32;
        let normalized_height = (r * g * b) / (255.0 * 255.0 * 255.0);
        normalized_height * Terrain::MAX_HEIGHT
    }

    fn calculate_normal(height_map: &DynamicImage, x: u32, z: u32) -> Vector3<f32> {
        let height_l = Terrain::get_height(height_map, if x == 0 {0} else {x-1} , z);
        let height_r = Terrain::get_height(height_map, x + 1, z);
        let height_d = Terrain::get_height(height_map, x, if z == 0 {0} else {z-1});
        let height_u = Terrain::get_height(height_map, x, z + 1);
        let normal = Vector3 {
            x: height_l - height_r,
            y: 2.0,
            z: height_d - height_u,
        };
        normal.normalize()
    }

    pub fn get_height_of_terrain(heights: &Heights, world_x: f32, world_z: f32, terrain_origin_x: f32, terrain_origin_z: f32) -> f32 {
        let terrain_x = world_x - terrain_origin_x;
        let terrain_z = world_z - terrain_origin_z;
        let grid_square_size = Terrain::SIZE as f32 / (Terrain::VERTEX_COUNT as f32 - 1.0);
        let grid_x = (terrain_x / grid_square_size).floor() as i32;
        let grid_z = (terrain_z / grid_square_size).floor() as i32;

        if grid_x >= Terrain::VERTEX_COUNT as i32 - 1 || grid_z >= Terrain::VERTEX_COUNT as i32 - 1 || grid_x < 0 || grid_z < 0 {
            return 0.0;
        }

        // calculate where our coord in the current cell and normalize
        let normalized_cell_coord_x = (terrain_x % grid_square_size) / grid_square_size;
        let normalized_cell_coord_z = (terrain_z % grid_square_size) / grid_square_size;
        if normalized_cell_coord_x <= (1.0 - normalized_cell_coord_z) {
            return Terrain::barry_centric(Vector3{x: 0.0, y: heights[grid_x as usize][grid_z as usize], z: 0.0},
                                         Vector3{x: 1.0, y: heights[(grid_x + 1) as usize][grid_z as usize], z: 0.0},
                                         Vector3{x: 0.0, y: heights[grid_x as usize][(grid_z + 1) as usize], z: 1.0},
                                         Vector2{x: normalized_cell_coord_x, y: normalized_cell_coord_z});
        } else {
            return Terrain::barry_centric(Vector3{x: 1.0, y: heights[(grid_x + 1) as usize][grid_z as usize], z: 0.0},
                                         Vector3{x: 1.0, y: heights[(grid_x + 1) as usize][(grid_z + 1) as usize], z: 1.0},
                                         Vector3{x: 0.0, y: heights[grid_x as usize][(grid_z + 1) as usize], z: 1.0},
                                         Vector2{x: normalized_cell_coord_x, y: normalized_cell_coord_z});
        }

    }

    fn barry_centric(p1: Vector3<f32>, p2: Vector3<f32>, p3: Vector3<f32>, pos: Vector2<f32>) -> f32 {
        let det = (p2.z - p3.z) * (p1.x - p3.x) + (p3.x - p2.x) * (p1.z - p3.z);
        let l1 = ((p2.z - p3.z) * (pos.x - p3.x) + (p3.x - p2.x) * (pos.y - p3.z)) / det;
        let l2 = ((p3.z - p1.z) * (pos.x - p3.x) + (p1.x - p3.x) * (pos.y - p3.z)) / det;
        let l3 = 1.0 - l1 - l2;
        return l1 * p1.y + l2 * p2.y + l3 * p3.y;
    }

    pub fn get_height_for_position(grid_position: &ReadStorage<GridPosition>, terrain_physics: &ReadStorage<TerrainPhysics>, x: f32, z: f32) -> f32 {
        let mut height = 0.0;
        use super::specs::Join;
        for (grid_position, terrain_physics) in (grid_position, terrain_physics).join() {
            let terrain_size = Terrain::SIZE as i32;
            let terrain_x_orig = (grid_position.x * terrain_size) as f32;
            let terrain_z_orig = (grid_position.z * terrain_size) as f32;
            if x >= terrain_x_orig && x < terrain_x_orig + terrain_size as f32 &&
                z >= terrain_z_orig && z < terrain_z_orig + terrain_size as f32 {
                height = Terrain::get_height_of_terrain(&terrain_physics.heights, x, z, terrain_x_orig, terrain_z_orig);
                break
            }
        }
        height
    }
}
