use glfw;
use glfw::{Context,Action,Key};
use std::sync::mpsc::Receiver;
use camera::Camera;

use gl;

pub fn new_window(width: u32, height: u32, title: &str) -> (glfw::Glfw,glfw::Window,Receiver<(f64, glfw::WindowEvent)>)
{
    // initialize glfw
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("failed to create glfw");
    // set opengl version to 3
    glfw.window_hint(glfw::WindowHint::ContextVersion(3,3));
    // tell glfw to use the core profile
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    //create window
    let (mut window, events) = glfw.create_window(width,height,title,glfw::WindowMode::Windowed)
        .expect("Failed to create window");

    // make window the current context
    window.make_current(); 
    // get key input 
    window.set_key_polling(true);
    // properly handle window resizing
    window.set_framebuffer_size_polling(true);
   
    // get mouse input
    window.set_cursor_pos_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    unsafe 
    {
        gl::Enable(gl::DEPTH_TEST);
    }

    (glfw,window,events)
}

pub fn process_events(window: &mut glfw::Window, events: &Receiver<(f64,glfw::WindowEvent)>,camera: &mut Camera)
{
    for (_,event) in glfw::flush_messages(events)
    {
        match event
        {
            // exit if esc was pressed 
            glfw::WindowEvent::Key(Key::Escape,_,Action::Press,_) => window.set_should_close(true),
            // movement 
            glfw::WindowEvent::Key(Key::Up,_,_,_) => camera.move_to(0),
            glfw::WindowEvent::Key(Key::Down,_,_,_) => camera.move_to(1),
            glfw::WindowEvent::Key(Key::Left,_,_,_) => camera.move_to(2),
            glfw::WindowEvent::Key(Key::Right,_,_,_) => camera.move_to(3),
            glfw::WindowEvent::CursorPos(xpos, ypos) => camera.rotate(xpos as f32,ypos as f32),
            _ => {}
        }
    }
}
