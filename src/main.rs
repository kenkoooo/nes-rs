extern crate gl;
extern crate glfw;
extern crate nes_rs;

use glfw::{Action, Context, Key};
use nes_rs::nes;
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
}

fn start(path: &str, window: &mut glfw::Window) {
    let hash = "a".to_owned();
    let console = nes::console::Console::new(path);
}
