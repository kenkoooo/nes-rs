use crate::nes::console::Console;
use crate::ui::util;
use gl;
use glfw::Window;

pub struct GameView {
    pub window: Window,
    console: Console,
    title: String,
    hash: String,
    texture: u32,
    record: bool,
}

impl GameView {
    pub fn new(window: Window, console: Console, title: String, hash: String) -> Self {
        let texture = util::create_texture();
        GameView {
            window: window,
            console: console,
            title: title,
            hash: hash,
            texture: texture,
            record: false,
        }
    }

    pub fn enter(&mut self) {
        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
        }
        self.window.set_title(&self.title);
        self.console.reset();

        if self.console.cartridge.battery != 0 {
            unimplemented!();
        }
    }

    pub fn update(&mut self, t: f64, dt: f64) {
        unimplemented!()
    }
}
