use crate::nes::cartridge;
use crate::nes::{Cartridge, Controller, Mapper, APU, CPU, PPU};
use std::io;

pub struct Console {
    cpu: CPU,
    apu: APU,
    ppu: PPU,
    pub cartridge: Cartridge,
    controller1: Controller,
    controller2: Controller,
    mapper: Mapper,
    ram: [u8; 2048],
}

impl Console {
    pub fn new(path: &str) -> io::Result<Console> {
        let cartridge = cartridge::load_nes_file(path)?;
        let ram = [0; 2048];
        let controller1 = Controller::new();
        let controller2 = Controller::new();
        let mapper = Mapper::new(&cartridge);

        unimplemented!();
    }

    pub fn reset(&mut self) {
        unimplemented!();
    }
}
