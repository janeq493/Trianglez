extern crate glfw;
extern crate gl;
extern crate cgmath;
extern crate image;

use glfw::{Context,Action,Key};
use self::gl::types::*;

use std::ptr;
use std::mem;
use std::os::raw::c_void;

use cgmath::*;
use cgmath::prelude::*;

use image::GenericImage;

mod shader;
use shader::Shader;

mod helpers;
use helpers::*;

mod camera;
use camera::Camera;

mod mesh;
use mesh::*;

fn main() 
{
    // load window and glfw
    let (mut glfw,mut window,mut events) = new_window(800,600,"hello");

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);


    unsafe 
    {
        gl::Enable(gl::DEPTH_TEST);
    }
    
    // creata shader program and add shaders
    let mut shader = Shader::new();

    shader.add_shader("shaders/vs.glsl",gl::VERTEX_SHADER);
    shader.add_shader("shaders/fs.glsl",gl::FRAGMENT_SHADER);
    shader.build_program();


    let verteces  =
        vec![
        vert { pos:vec3(-0.5,-0.5,-0.5) ,col:vec3(1.0,0.0,0.1) },
        vert { pos:vec3(-0.5,0.5,-0.5),col:vec3(0.0,1.0,0.3) },
        vert { pos:vec3(0.5,0.5,-0.5),col:vec3(0.3,0.1,1.0) },
        vert { pos:vec3(0.5,-0.5,-0.5),col:vec3(0.5,0.2,0.1) },
        vert { pos:vec3(0.,0.,0.5),col:vec3(1.0,0.0,0.7) },
        ];
    let indices  =
        vec![
        0,1,3,
        3,2,1,
        0,1,4,
        1,2,4,
        2,3,4,
        3,0,4,
        ];
    let tetrahed = Mesh::new_color(verteces,indices);
    
    let positions: [Vector3<f32>;4] =
        [
        vec3(3.0,-2.0,1.0),
        vec3(-3.0,-7.0,-2.0),
        vec3(0.0,-3.0,-8.0),
        vec3(-5.0,1.0,5.0),
        ];

    let mut camera = Camera::new(Point3::new(0.0,0.0,3.0),Vector3::new(0.0,0.0,-1.0),Vector3::new(0.0,1.0,0.0));
    let proj = perspective(Deg(80.0),800.0/600.0,0.1,100.0);
    while !window.should_close()
    {
        let time =glfw.get_time() as f32;
        //input
        process_events(&mut window,&events,&mut camera);
        let view = camera.get_view();
        //let mut model = Matrix4::identity();
        //rendering
        unsafe 
        {
            shader.set_mat4("view_mat",&view);
            shader.set_mat4("proj_mat",&proj);
            // Set color of the background
            gl::ClearColor(0.2,0.2,0.3,1.0);
            // clear the screen
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            for (i,p) in positions.iter().enumerate()
            {
                let model = Matrix4::from_translation(*p)
                    *Matrix4::from_axis_angle(vec3((time*i as f32 +i as f32).sin(),1.0,0.0).normalize(),Rad(time+20.0*i as f32));
                        
                shader.set_mat4("mod_mat",&model);

                tetrahed.draw(&mut shader);

            }
        }

        // output buffer that has been used to draw during this iteration to the screen
        window.swap_buffers();
        // handle events
        glfw.poll_events();
    }
}


