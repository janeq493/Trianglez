#![allow(dead_code)]
use self::gl::types::*;
use gl;

use std::fs::File;
use std::io::prelude::*;

use std::ffi::CString;
use std::str;
use std::ptr;

use cgmath::{Matrix, Matrix4, Vector3};
use cgmath::prelude::*;

pub struct Shader
{
    shaders: Vec<u32>,
    id: u32
}
#[allow(non_snake_case)]
impl Shader
{
    pub fn new() -> Shader
    {
        Shader
        {
            shaders: Vec::new(),
            id: 0,

        }
    }

    pub fn add_shader(&mut self, path: &str, shaderType: u32)
    {
        let mut source = String::new();
        let mut f = File::open(path).expect("file not found");

        f.read_to_string(&mut source).expect("failed to read file");

        unsafe
        {
            
            let shader = gl::CreateShader(shaderType);

            gl::ShaderSource(shader,1,&CString::new(source.as_bytes()).expect("Failed to parse VertexShaderSource as cstring").as_ptr(),ptr::null());

            gl::CompileShader(shader);

            let mut success = gl::FALSE as GLint;
            let mut infoLog = Vec::with_capacity(1024);
            infoLog.set_len(1023); // without trailing null char
            infoLog.clear();

            gl::GetShaderiv(shader,gl::COMPILE_STATUS,&mut success);
            if success != gl::TRUE as GLint
            {
                gl::GetShaderInfoLog(shader,1024,ptr::null_mut(),infoLog.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::COMPILATION_FAILED\n{}", str::from_utf8(&infoLog).unwrap());
            }
            self.shaders.push(shader);
        }
    }

    pub fn build_program(&mut self)
    {
        unsafe
        {
            let shaderProgram = gl::CreateProgram();
            for shader in &mut self.shaders
            {
                gl::AttachShader(shaderProgram,*shader);
            }
            gl::LinkProgram(shaderProgram);

            let mut success = gl::FALSE as GLint;
            let mut infoLog = Vec::with_capacity(1024);
            infoLog.set_len(1023);
            infoLog.clear();

            gl::GetShaderiv(shaderProgram,gl::LINK_STATUS,&mut success);
            if success != gl::TRUE as GLint
            {
                gl::GetShaderInfoLog(shaderProgram,1024,ptr::null_mut(),infoLog.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&infoLog).unwrap());
            }
            for shader in &mut self.shaders
            {
                gl::DeleteShader(*shader);

            }
            self.id = shaderProgram;
        }
    }
    pub unsafe fn use_prog(&mut self)
    {
        gl::UseProgram(self.id);
    }

    pub unsafe fn set_bool(&self,name: &str, val: bool)
    {
        let name = CString::new(name).unwrap();
        gl::Uniform1i(gl::GetUniformLocation(self.id,name.as_ptr()),val as i32);
    }
    pub unsafe fn set_int(&self,name: &str, val: i32)
    {
        let name = CString::new(name).unwrap();
        gl::Uniform1i(gl::GetUniformLocation(self.id,name.as_ptr()),val);
    }
    pub unsafe fn set_float(&self,name: &str, val: f32)
    {
        let name = CString::new(name).unwrap();
        gl::Uniform1f(gl::GetUniformLocation(self.id,name.as_ptr()),val);
    }
    pub unsafe fn set_vector3(&self,name: &str, val: &Vector3<f32>)
    {
        let name = CString::new(name).unwrap();
        gl::Uniform3fv(gl::GetUniformLocation(self.id,name.as_ptr()),1,val.as_ptr());
    }
    pub unsafe fn set_vec3(&self,name: &str, x: f32, y: f32, z: f32)
    {
        let name = CString::new(name).unwrap();
        gl::Uniform3f(gl::GetUniformLocation(self.id,name.as_ptr()),x,y,z);
    }
    pub unsafe fn set_mat4(&self,name: &str, mat: &Matrix4<f32>)
    {
        let name = CString::new(name).unwrap();
        gl::UniformMatrix4fv(gl::GetUniformLocation(self.id,name.as_ptr()),1, gl::FALSE, mat.as_ptr());
    }



}
