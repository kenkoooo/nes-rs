extern crate gl;
extern crate glfw;
extern crate nes_rs;

use glfw::{Action, Context, Key};
use nes_rs::nes;
use nes_rs::ui::GameView;
use std::env;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 240;
const SCALE: u32 = 3;
const TITLE: &str = "NES";

fn main() {
    let args: Vec<String> = env::args().collect();
    run(&args[1]);
}

fn run(path: &str) {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(2));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(1));
    let (mut window, events) = glfw
        .create_window(
            WIDTH * SCALE,
            HEIGHT * SCALE,
            TITLE,
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s));

    let hash = "a".to_owned();
    let console = nes::console::Console::new(path);
    let mut view = GameView::new(window, console, path.to_owned(), hash);
    view.enter();

    let mut timestamp = unsafe { glfw::ffi::glfwGetTime() };
    while !view.window.should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        let now = unsafe { glfw::ffi::glfwGetTime() };
        let dt = now - timestamp;
        timestamp = now;
        view.update(now, dt);
        view.window.swap_buffers();
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut view.window, event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
