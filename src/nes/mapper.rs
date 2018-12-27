use crate::nes::Console;

pub enum Mapper {
    Mapper1 {},
    Mapper2 {},
    Mapper3 {},
    Mapper4 {},
}

impl Mapper {
    pub fn new(console: &Console) -> Self {
        match console.cartridge.mapper {
            0 => {}
            1 => {}
            2 => {}
            3 => {}
            4 => {}
            7 => {}
            225 => {}
            _ => unreachable!(),
        }
    }
}
