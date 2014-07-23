extern crate native;
extern crate glfw;
extern crate core;

use glfw::Context;

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let option = glfw.create_window(400, 300, "Hello World", glfw::Windowed);

    match option {
        Some((window, events)) => {
            window.set_key_polling(true);
            window.make_current();

            while !window.should_close() {
                glfw.poll_events();
                for (_, event) in glfw::flush_messages(&events) {
                    handle_window_event(&window, event);
                }
            }
        },
        None => {}
    }
}

fn handle_window_event(window: &glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => {
            window.set_should_close(true);
        }
        _ => {}
    }
}
