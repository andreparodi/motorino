use super::resources::ResourceLoader;
use super::gl::types::*;
use super::gl;
use super::tobj;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::path::Path;
use std::rc::Rc;
use super::image::GenericImageView;
use super::components::Texture;
use super::components::SimpleTexture;
use super::components::RawModel;
use super::image::DynamicImage;
use std::collections::HashMap;

pub struct CubeMapDefinition {
    pub back: String,
    pub bottom: String,
    pub front: String,
    pub left: String,
    pub right: String,
    pub top: String
}

pub struct Loader {
    resource_loader: Rc<ResourceLoader>,
    pub vaos: Vec<GLuint>,
    pub vbos: Vec<GLuint>,
    pub textures: Vec<GLuint>,
    pub textures_paths: HashMap<GLuint, String>
}

impl Loader {

    pub fn new(resource_loader: Rc<ResourceLoader>) -> Loader {
        Loader {
            resource_loader,
            vaos: Vec::new(),
            vbos: Vec::new(),
            textures: Vec::new(),
            textures_paths: HashMap::new()
        }
    }

    pub fn load_to_vao(&mut self, positions: &[f32], texture_coords: &[f32], normals: &[f32], indices: &[u32]) -> RawModel {
        let vao_id = Loader::create_vao();
        self.vaos.push(vao_id);
        self.bind_indices_buffer(indices);
        self.store_data_in_attribute_list(0, 3, &positions);
        self.store_data_in_attribute_list(1, 2, &texture_coords);
        self.store_data_in_attribute_list(2, 3, &normals);
        Loader::unbind_vao();
        RawModel {
            vao_id: vao_id,
            vertex_count: indices.len()
        }
    }

    pub fn load_positions_to_vao(&mut self, positions: &[f32], dimension: i32) -> RawModel {
        let vao_id = Loader::create_vao();
        self.vaos.push(vao_id);
        self.store_data_in_attribute_list(0, dimension, &positions);
        Loader::unbind_vao();
        RawModel {
            vao_id: vao_id,
            vertex_count: positions.len()/dimension as usize
        }
    }


    pub fn load_from_obj(&mut self, path: &str) -> RawModel {
        let obj_path = self.resource_loader.to_real_path(&Path::new(path));
        let (models, _materials) = tobj::load_obj(obj_path.as_path()).unwrap();
        for (_i, m) in models.iter().enumerate() {
            let indices = &m.mesh.indices;
            let positions = &m.mesh.positions;
            let texture_coords = &m.mesh.texcoords;
            let normals = &m.mesh.normals;
            return self.load_to_vao(positions, texture_coords, normals, indices);
        }
        tobj::load_mtl(obj_path.as_path()).unwrap();
        return RawModel{vao_id:0, vertex_count: 0};
    }

    pub fn load_simple_texture(&mut self, path: &str, reflectivity: f32, shine_damper: f32) -> Result<SimpleTexture, String> {
        let texture = self.load_texture(path);
        return Ok(SimpleTexture{texture_id: texture, reflectivity: reflectivity, shine_damper: shine_damper});
    }

    pub fn load_terrain_texture(&mut self, path: &str) -> Texture {
        Texture { texture_id: self.load_texture(path)}
    }

    fn load_texture(&mut self, path: &str) -> GLuint {
        let mut texture = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
            self.textures.push(texture);
            self.textures_paths.insert(texture, path.to_owned());
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            // TODO handle errors
            let texture = self.resource_loader.load_image(path).unwrap();
            let data = texture.raw_pixels();

            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::RGB as i32,
                           texture.width() as i32,
                           texture.height() as i32,
                           0,
                           gl::RGB,
                           gl::UNSIGNED_BYTE,
                           &data[0] as *const u8 as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        return texture;
    }

    pub fn load_cube_map(&mut self, cube_map_def: &CubeMapDefinition) -> Texture {
        let mut texture = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
            self.textures.push(texture);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, texture);

            self.load_cube_map_face(&cube_map_def.right, gl::TEXTURE_CUBE_MAP_POSITIVE_X);
            self.load_cube_map_face(&cube_map_def.left, gl::TEXTURE_CUBE_MAP_NEGATIVE_X);
            self.load_cube_map_face(&cube_map_def.top, gl::TEXTURE_CUBE_MAP_POSITIVE_Y);
            self.load_cube_map_face(&cube_map_def.bottom, gl::TEXTURE_CUBE_MAP_NEGATIVE_Y);
            self.load_cube_map_face(&cube_map_def.back, gl::TEXTURE_CUBE_MAP_POSITIVE_Z);
            self.load_cube_map_face(&cube_map_def.front, gl::TEXTURE_CUBE_MAP_NEGATIVE_Z);

            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
        }
        Texture{texture_id: texture}
    }

    fn load_cube_map_face(&mut self, path: &str, face: u32) {
        unsafe {
            let texture = self.resource_loader.load_image(path).unwrap();
            let texture = match texture {
                DynamicImage::ImageRgba8(texture) => texture,
                texture => texture.to_rgba()
            };
            gl::TexImage2D(
                face,
                0,
                gl::RGBA as i32,
                texture.width() as i32,
                texture.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                texture.into_raw().as_ptr() as *const c_void);
        }
    }

    fn bind_indices_buffer(&mut self, indices: &[u32]) {
        let mut vbo_id = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo_id);
            self.vbos.push(vbo_id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vbo_id);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
                &indices[0] as *const u32 as *const c_void,
                gl::STATIC_DRAW);
        }
    }

    fn create_vao() -> GLuint {
        let mut vao_id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao_id);
            gl::BindVertexArray(vao_id);
        }
        return vao_id;
    }

    fn unbind_vao() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    fn store_data_in_attribute_list(&mut self, attribute_id: GLuint, coordinate_size: i32, data: &[f32]) {
        let mut vbo_id = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo_id);
            self.vbos.push(vbo_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                           &data[0] as *const f32 as *const c_void,
                           gl::STATIC_DRAW);
            gl::VertexAttribPointer(attribute_id, coordinate_size, gl::FLOAT, gl::FALSE, coordinate_size * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
            gl::EnableVertexAttribArray(attribute_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

impl Drop for Loader {
    fn drop(&mut self) {
        for vao_id in &self.vaos {
            // check first parameter
            unsafe { gl::DeleteVertexArrays(1, vao_id); }
        }
        for vbo_id in &self.vbos {
            unsafe { gl::DeleteBuffers(1, vbo_id); }
        }
        for texture in &self.textures {
            unsafe { gl::DeleteTextures(1, texture); }
        }
    }
}
