use crate::nes::apu::APU;
use crate::nes::cpu::CPU;
use crate::nes::ppu::PPU;

pub struct Console {
    cpu: CPU,
    apu: APU,
    ppu: PPU,
}

impl Console {
    pub fn new(path: &str) -> Self {
        unimplemented!()
    }
}
