#![feature(globs)]

extern crate core;
extern crate gl;
extern crate glfw;
extern crate native;

// use gl::types::*;
use glfw::Context;

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Select OpenGL mode
    glfw.window_hint(glfw::ContextVersion(3, 2));
    glfw.window_hint(glfw::OpenglForwardCompat(true));
    glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));

    let option = glfw.create_window(400, 300, "Hello World", glfw::Windowed);

    match option {
        Some((window, events)) => {
            window.set_key_polling(true);
            window.make_current();

            gl::load_with( |s| glfw.get_proc_address(s) );
            glfw.set_swap_interval(1);

            while !window.should_close() {
                glfw.poll_events();

                clear();

                for (_, event) in glfw::flush_messages(&events) {
                    handle_window_event(&window, event);
                }

                window.swap_buffers();
            }
        },
        None => {}
    }
}

fn clear() {
    gl::ClearColor(0.3, 0.3, 0.3, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);
}

fn handle_window_event(window: &glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => {
            window.set_should_close(true);
        }
        _ => {}
    }
}
