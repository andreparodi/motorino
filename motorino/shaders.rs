use std::ptr;
use std::ffi::{CStr, CString};
use super::cgmath::{Matrix, Matrix4, Vector3};
use super::cgmath::prelude::*;
use super::gl;
use super::gl::types::*;
use super::resources::ResourceLoader;

// =================================================================================================
pub struct Shader {
    id: GLuint
}

impl Shader {
    pub fn from_source(
        source: &CStr,
        kind: GLenum
    ) -> Result<Shader, String> {
        let id = Shader::shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    fn shader_from_source(source: &CStr, kind: GLenum) -> Result<GLuint, String> {
        let id = unsafe { gl::CreateShader(kind) };
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), ptr::null());
            gl::CompileShader(id);
            Shader::check_compile_error(id, kind);
        }

        // handle errors
        Ok(id)
    }

    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    unsafe fn check_compile_error(id: GLuint, kind: GLenum) {
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(1024);
        info_log.set_len(1024 - 1); // subtract 1 to skip the trailing null character
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(id, 1024, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            println!("ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n \
                              -- --------------------------------------------------- -- ",
                     kind,
                     String::from_utf8(info_log).unwrap());
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }

}
// =================================================================================================

// =================================================================================================
pub struct ShaderProgram {
    id: GLuint
}

impl ShaderProgram {

    pub fn from_shader_files(resource_loader: &ResourceLoader, vertex_shader: &str, fragment_shader: &str) -> Result<ShaderProgram, String> {
        let c_str_vert: CString = resource_loader.load_cstring(vertex_shader)?;
        let vert = Shader::from_vert_source(&c_str_vert)?;

        let c_str_frag = resource_loader.load_cstring(fragment_shader)?;
        let frag = Shader::from_frag_source(&c_str_frag)?;

        let program = ShaderProgram::from_shaders(&[vert, frag]);
        return program;
    }


    pub fn from_shaders(shaders: &[Shader]) -> Result<ShaderProgram, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe { gl::AttachShader(program_id, shader.id()); }
        }

        unsafe {
            gl::LinkProgram(program_id);
            ShaderProgram::check_compile_error(program_id)

        }

        // handle errros

        for shader in shaders {
            unsafe { gl::DetachShader(program_id, shader.id()); }
        }


        Ok(ShaderProgram { id: program_id})
    }

    #[allow(dead_code)]
    pub fn get_uniform_location(&self, uniform_name: &String) -> Result<i32, String> {
        unsafe {
            return Ok(gl::GetUniformLocation(self.id, CString::new(uniform_name.as_bytes()).unwrap().as_ptr()));
        }
    }

    #[allow(dead_code)]
    pub fn set_bool(&self, name: &CStr, value: bool) {
        unsafe {
            gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value as i32);
        }
    }

    #[allow(dead_code)]
    pub fn set_int(&self, name: &CStr, value: i32) {
        unsafe {
            gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value);
        }
    }

    #[allow(dead_code)]
    pub fn set_float(&self, name: &CStr, value: f32) {
        unsafe {
            gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr()), value);
        }
    }

    pub fn set_vector3(&self, name: &CStr, value: &Vector3<f32>) {
        unsafe {
            gl::Uniform3fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, value.as_ptr());
        }
    }

    #[allow(dead_code)]
    pub fn set_vec3(&self, name: &CStr, x: f32, y: f32, z: f32) {
        unsafe {
            gl::Uniform3f(gl::GetUniformLocation(self.id, name.as_ptr()), x, y, z);
        }
    }

    pub fn set_mat4(&self, name: &CStr, mat: &Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, gl::FALSE, mat.as_ptr());
        }
    }

    #[allow(dead_code)]
    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn start(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn stop(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    unsafe fn check_compile_error(id: GLuint) {
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(1024);
        info_log.set_len(1024 - 1); // subtract 1 to skip the trailing null character
        gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(id, 1024, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            println!("ERROR::PROGRAM_LINKING_ERROR: {}\n \
                      -- --------------------------------------------------- -- ",
                     String::from_utf8(info_log).unwrap());
        }
    }

}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            // should we detach shaders first
            gl::DeleteProgram(self.id);
        }
    }
}
// =================================================================================================
