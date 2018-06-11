extern crate glfw;
extern crate gl;
extern crate cgmath;
extern crate image;
extern crate tobj;

use glfw::Context;

use cgmath::*;

mod shader;
use shader::Shader;

mod helpers;
use helpers::*;

mod camera;
use camera::Camera;

mod mesh;
use mesh::*;

mod model;
use model::*;

fn main() 
{
    let (width,height) = (800,600);

    // load window and glfw
    let (mut glfw,mut window,events) = new_window(width,height,"hello");


    // creata shader program and add shaders
    let mut shader = Shader::new();
    
    shader.add_shader("shaders/light/vs.glsl",gl::VERTEX_SHADER);
    shader.add_shader("shaders/light/fs.glsl",gl::FRAGMENT_SHADER);
    shader.build_program();
    
    let mut tes = Model::new("models/treeman.obj");

    let positions: [Vector3<f32>;4] =
        [
        vec3(3.0,-2.0,1.0),
        vec3(-3.0,-7.0,-2.0),
        vec3(0.0,-3.0,-8.0),
        vec3(-5.0,1.0,5.0),
        ];

    // initialize camera
    let mut camera = Camera::new(Point3::new(0.0,0.0,3.0),Vector3::new(0.0,0.0,-1.0));
    // create perspective matrix
    let proj = perspective(Deg(80.0),width as f32/height as f32,0.1,100.0);

    // main game loop
    while !window.should_close()
    {
        let time = glfw.get_time() as f32;
        
        // handle input
        process_events(&mut window,&events,&mut camera);

        // get view matrix
        let view = camera.get_view();
        
        //rendering
        unsafe 
        {
            // Set color of the background
            gl::ClearColor(0.05,0.05,0.08,1.0);

            // clear the screen
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            shader.use_prog();
            shader.set_mat4("view_mat",&view);
            shader.set_mat4("proj_mat",&proj);

            // loop over all objects that are supposed to be rendered and render them
            for (i,p) in positions.iter().enumerate()
            {
                // set correct position and make them rotate over time
                let model = Matrix4::from_translation(*p)
                    *Matrix4::from_axis_angle(vec3((time*i as f32 +i as f32).sin(),1.0,0.0).normalize(),Rad(time+20.0*i as f32));

                shader.set_mat4("mod_mat",&model);

                tes.draw(&mut shader);
            }
        }

        // swap the output buffer that has been used to draw during this iteration to the screen
        window.swap_buffers();
        
        glfw.poll_events();
    }
}
