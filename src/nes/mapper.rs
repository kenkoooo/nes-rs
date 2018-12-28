use crate::nes::Cartridge;

use crate::nes::mapper1::Mapper1;
use crate::nes::mapper2::Mapper2;
use crate::nes::mapper225::Mapper225;
use crate::nes::mapper3::Mapper3;
use crate::nes::mapper4::Mapper4;
use crate::nes::mapper7::Mapper7;

pub enum Mapper {
    Mapper1(Mapper1),
    Mapper2(Mapper2),
    Mapper3(Mapper3),
    Mapper4(Mapper4),
    Mapper7(Mapper7),
    Mapper225(Mapper225),
}

impl Mapper {
    pub fn new(cartridge: &Cartridge) -> Self {
        match cartridge.mapper {
            0 => Mapper::Mapper2(Mapper2::new(cartridge)),
            1 => Mapper::Mapper1(Mapper1::new(cartridge)),
            2 => Mapper::Mapper2(Mapper2::new(cartridge)),
            3 => Mapper::Mapper3(Mapper3::new(cartridge)),
            4 => Mapper::Mapper4(Mapper4::new(cartridge)),
            7 => Mapper::Mapper7(Mapper7::new(cartridge)),
            225 => Mapper::Mapper225(Mapper225::new(cartridge)),
            _ => unreachable!(),
        }
    }
}
