use crate::nes::cartridge;
use crate::nes::{Cartridge, Controller, APU, CPU, PPU};
use std::io;

pub struct Console {
    cpu: CPU,
    apu: APU,
    ppu: PPU,
    pub cartridge: Cartridge,
}

impl Console {
    pub fn new(path: &str) -> io::Result<Console> {
        let cartridge = cartridge::load_nes_file(path)?;
        let mut ram = [0; 2048];
        let controller1 = Controller::new();
        let controller2 = Controller::new();

        unimplemented!();
    }

    pub fn reset(&mut self) {
        unimplemented!();
    }
}
