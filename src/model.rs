use cgmath::{vec2, vec3};
use gl;
use image;
use image::DynamicImage::*;
use image::GenericImage;
use tobj;

use std::path::Path;

use shader::*;
use mesh::*;

#[derive(Default)]
pub struct Model
{
    meshes: Vec<Mesh>,
}

impl Model
{
    pub fn new(path: &str) -> Model
    {
       let mut mdl = Model::default();
       mdl.loadModel(path);
       mdl
    }
    fn loadModel(&mut self,path:&str)
    {
        let (models,materials) = tobj::load_obj(Path::new(path)).unwrap();
        for model in &models
        {
            let mut vertices = Vec::new();
            let mut mesh = &model.mesh;
            let indices = mesh.indices.clone();
            let (pos,norm,tex) = (&mesh.positions,&mesh.normals,&mesh.texcoords);
            let nov = mesh.positions.len()/3;
            for i in 0..nov
            {
            println!("{}",nov);
                vertices.push( vert 
                               {
                                   pos: vec3(pos[i],pos[i+1],pos[i+2]),
                                   norm: vec3(norm[i],norm[i+1],norm[i+2]),
                                   //tex: vec2(tex[i],tex[i+1])
                               } );
            }
            self.meshes.push(Mesh::new(vertices,indices));

        }
    }
    pub fn draw(&self, mut shader: &mut Shader)
    {
        unsafe
        {
            for m in &self.meshes
            {
                m.draw(&mut shader);
            }
        }
    }
}
