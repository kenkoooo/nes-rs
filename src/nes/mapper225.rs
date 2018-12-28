use crate::nes::Cartridge;

// https://github.com/asfdfdfd/fceux/blob/master/src/boards/225.cpp
// https://wiki.nesdev.com/w/index.php/INES_Mapper_225

pub struct Mapper225 {
    chr_bank: i32,
    prg_bank1: i32,
    prg_bank2: i32,
}

impl Mapper225 {
    pub fn new(cartridge: &Cartridge) -> Self {
        let prg_banks = (cartridge.prg.len() / 0x4000) as i32;
        Mapper225 {
            chr_bank: 0,
            prg_bank1: 0,
            prg_bank2: prg_banks - 1,
        }
    }
}
