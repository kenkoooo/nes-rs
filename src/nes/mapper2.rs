use crate::nes::Cartridge;

pub struct Mapper2 {
    prg_banks: i32,
    prg_bank1: i32,
    prg_bank2: i32,
}

impl Mapper2 {
    pub fn new(cartridge: &Cartridge) -> Self {
        let prg_banks = (cartridge.prg.len() / 0x4000) as i32;
        Mapper2 {
            prg_banks: prg_banks,
            prg_bank1: 0,
            prg_bank2: prg_banks - 1,
        }
    }
}
