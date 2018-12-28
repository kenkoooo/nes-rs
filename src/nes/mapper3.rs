use crate::nes::Cartridge;

pub struct Mapper3 {
    chr_bank: i32,
    prg_bank1: i32,
    prg_bank2: i32,
}

impl Mapper3 {
    pub fn new(cartridge: &Cartridge) -> Self {
        let prg_banks = (cartridge.prg.len() / 0x4000) as i32;
        Mapper3 {
            chr_bank: 0,
            prg_bank1: 0,
            prg_bank2: prg_banks - 1,
        }
    }
}
