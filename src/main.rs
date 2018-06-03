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


    let verteces: [f32;6*5] =
        [
        -0.5,-0.5,-0.5,1.0,0.0,0.1,
        -0.5,0.5,-0.5,0.0,1.0,0.3,
        0.5,0.5,-0.5,0.3,0.1,1.0,
        0.5,-0.5,-0.5,0.5,0.2,0.1,
        0.,0.,0.5,1.0,0.0,0.7,
        ];
    let indices: [i32;3*6] =
        [
        0,1,3,
        3,2,1,
        0,1,4,
        1,2,4,
        2,3,4,
        3,0,4,
        ];
    let VAO = unsafe
    { 
        // generate buffers and vao
        let  (mut VBO, mut VAO, mut EBO) = (0,0,0);
        gl::GenBuffers(1,&mut VBO);
        gl::GenBuffers(1,&mut EBO);
        gl::GenVertexArrays(1,&mut VAO);

        gl::BindVertexArray(VAO);

        gl::BindBuffer(gl::ARRAY_BUFFER,VBO);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (verteces.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &verteces[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW
                      );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER,EBO);

        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       (indices.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
                       &indices[0] as *const i32 as *const c_void,
                       gl::STATIC_DRAW
                      );
        gl::VertexAttribPointer(0,3,gl::FLOAT,gl::FALSE,(6* mem::size_of::<GLfloat>()) as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(1,3,gl::FLOAT,gl::FALSE,(6* mem::size_of::<GLfloat>()) as GLsizei, (3* mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(1);



        gl::BindBuffer(gl::ARRAY_BUFFER,0);

        gl::BindVertexArray(0);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER,0);
        VAO
    };
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

            // load shader program create earlier
            shader.use_prog();

            gl::BindVertexArray(VAO);

            for (i,p) in positions.iter().enumerate()
            {
                let model = Matrix4::from_translation(*p)
                    *Matrix4::from_axis_angle(vec3((time*i as f32 +i as f32).sin(),1.0,0.0).normalize(),Rad(time+20.0*i as f32));
                        
                shader.set_mat4("mod_mat",&model);

                gl::DrawElements(gl::TRIANGLES,verteces.len() as i32, gl::UNSIGNED_INT,ptr::null());
            }
        }

        // output buffer that has been used to draw during this iteration to the screen
        window.swap_buffers();
        // handle events
        glfw.poll_events();
    }
}


