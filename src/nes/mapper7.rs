use crate::nes::Cartridge;

pub struct Mapper7 {
    prg_bank: i32,
}

impl Mapper7 {
    pub fn new(cartridge: &Cartridge) -> Self {
        Mapper7 { prg_bank: 0 }
    }
}
