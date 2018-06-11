use std::os::raw::c_void;
use std::ptr;
use std::mem;

use cgmath::{Vector3,Vector2};
use cgmath::prelude::*;

use gl;
use self::gl::types::*;

use shader::Shader;

#[repr(C)]
pub struct vert
{
    pub pos: Vector3<f32>,
    pub norm:Vector3<f32>,
    pub tex: Vector2<f32>,
}

impl Default for vert
{
    fn default() -> Self
    {
        vert 
        {
            pos: Vector3::zero(),
            norm: Vector3::zero(),
            tex: Vector2::zero(),
        }
    }
}

#[allow(non_snake_case)]
pub struct Mesh
{
    vertices: Vec<vert>,
    indices: Vec<u32>,
    VAO: u32,
    VBO:u32,
    EBO:u32,
}

impl Mesh
{
    pub fn new(vertices: Vec<vert>,indices: Vec<u32>) -> Mesh
    {
        let mut mesh = Mesh 
        {
            vertices, indices,
            VAO:0,VBO:0,EBO:0,
        };
        unsafe { mesh.setup() };
        mesh
    }

    
    unsafe fn setup(&mut self)
    {
        gl::GenBuffers(1,&mut self.VBO);
        gl::GenBuffers(1,&mut self.EBO);
        gl::GenVertexArrays(1,&mut self.VAO);
        gl::BindVertexArray(self.VAO);

        gl::BindBuffer(gl::ARRAY_BUFFER,self.VBO);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (self.vertices.len() * mem::size_of::<vert>()) as isize,
                       &self.vertices[0] as *const vert as *const c_void,
                       gl::STATIC_DRAW
                      );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER,self.EBO);

        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       (self.indices.len() * mem::size_of::<u32>()) as isize,
                       &self.indices[0] as *const u32 as *const c_void,
                       gl::STATIC_DRAW
                      );

        gl::VertexAttribPointer(0,3,gl::FLOAT,gl::FALSE,(mem::size_of::<vert>()) as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);
        
        gl::VertexAttribPointer(1,3,gl::FLOAT,gl::FALSE,(mem::size_of::<vert>()) as GLsizei, (mem::size_of::<vert>()/2) as *const c_void);
        gl::EnableVertexAttribArray(1);

        gl::VertexAttribPointer(2,2,gl::FLOAT,gl::FALSE,(mem::size_of::<vert>()) as GLsizei, (mem::size_of::<Vector3<f32>>()*2) as *const c_void);
        gl::EnableVertexAttribArray(2);


        gl::BindBuffer(gl::ARRAY_BUFFER,0);
        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER,0);
    }
    pub unsafe fn draw(&self, shader: &mut Shader)
    {
        shader.use_prog();
        gl::BindVertexArray(self.VAO);

        gl::DrawElements(gl::TRIANGLES,self.indices.len() as i32, gl::UNSIGNED_INT,ptr::null());

        gl::BindVertexArray(0);
    }

}
