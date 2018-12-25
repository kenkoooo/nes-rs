use crate::nes::console::Console;
use glfw::Window;

pub struct GameView {
    window: Window,
    console: Console,
    title: String,
    hash: String,
    texture: u32,
    record: bool,
}
