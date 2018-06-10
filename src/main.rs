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

    //let mut shader2 = Shader::new();

    //shader2.add_shader("shaders/simple/vs.glsl",gl::VERTEX_SHADER);
    //shader2.add_shader("shaders/simple/fs.glsl",gl::FRAGMENT_SHADER);
    //shader2.build_program();

    //// verteces of figure to be rendered
    //let verteces  =
    //    vec![
    //    vert { pos:vec3(-0.5,-0.5,-0.5) ,col:vec3(1.0,0.5,0.1) },
    //    vert { pos:vec3(-0.5,0.5,-0.5),col:vec3(1.0,0.5,0.1) },
    //    vert { pos:vec3(0.5,0.5,-0.5),col:vec3(1.0,0.5,0.1) },
    //    vert { pos:vec3(0.5,-0.5,-0.5),col:vec3(1.0,0.5,0.1) },
    //    vert { pos:vec3(0.,0.,0.5),col:vec3(1.0,0.5,0.1) },
    //    ];
    //// and its indices
    //let indices  =
    //    vec![
    //    0,1,3,
    //    3,2,1,
    //    0,1,4,
    //    1,2,4,
    //    2,3,4,
    //    3,0,4,
    //    ];
    //// create mesh for tetrahedron from previously defined verteces
    //let tetrahed = Mesh::new_color(verteces,indices);

    //let verteces2 =
    //    vec![
    //    vert { pos:vec3(-0.5,-0.5,-0.5)  , ..Default::default() },
    //    vert { pos:vec3(-0.5,0.5,-0.5) , ..Default::default() },
    //    vert { pos:vec3(0.5,0.5,-0.5) , ..Default::default() },
    //    vert { pos:vec3(0.5,-0.5,-0.5) , ..Default::default() },
    //    vert { pos:vec3(-0.5,-0.5,0.5)  , ..Default::default() },
    //    vert { pos:vec3(-0.5,0.5,0.5) , ..Default::default() },
    //    vert { pos:vec3(0.5,0.5,0.5), ..Default::default()  },
    //    vert { pos:vec3(0.5,-0.5,0.5), ..Default::default() },
    //    ];
    //let indices2 =
    //    vec![
    //    0,1,2,
    //    2,3,0,
    //    0,4,1,
    //    1,5,4,
    //    5,1,2,
    //    2,6,5,
    //    2,6,7,
    //    7,3,2,
    //    3,0,4,
    //    4,7,3,
    //    4,5,6,
    //    6,7,4,
    //    ];
    //let cube = Mesh::new_color(verteces2,indices2);
    //    
    
    let mut tes = Model::new("models/test.obj");
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
            //for (i,p) in positions.iter().enumerate()
            //{
            //    // set correct position and make them rotate over time
            //    let model = Matrix4::from_translation(*p)
            //        *Matrix4::from_axis_angle(vec3((time*i as f32 +i as f32).sin(),1.0,0.0).normalize(),Rad(time+20.0*i as f32));

            //    shader.set_mat4("mod_mat",&model);

            //    // draw the figure
            //    tetrahed.draw(&mut shader);
            //}

            //shader2.use_prog();
            //shader2.set_mat4("view_mat",&view);
            //shader2.set_mat4("proj_mat",&proj);

            let model = Matrix4::from_translation(vec3(3.0,3.0,3.0));
            shader.set_mat4("mod_mat",&model);

            tes.draw(&mut shader);
        }
        // swap the output buffer that has been used to draw during this iteration to the screen
        window.swap_buffers();
        
        glfw.poll_events();
    }
}
