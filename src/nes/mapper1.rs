use crate::nes::Cartridge;

pub struct Mapper1 {
    shift_register: u8,
    control: u8,
    prg_mode: u8,
    chr_mode: u8,
    prg_bank: u8,
    chr_bank0: u8,
    chr_bank1: u8,
    prg_offsets: [i32; 2],
    chr_offsets: [i32; 2],
}

impl Mapper1 {
    pub fn new(cartridge: &Cartridge) -> Self {
        let mut mapper = Mapper1 {
            shift_register: 0x10,
            control: 0,
            prg_mode: 0,
            chr_mode: 0,
            prg_bank: 0,
            chr_bank0: 0,
            chr_bank1: 0,
            prg_offsets: [0; 2],
            chr_offsets: [0; 2],
        };
        let offset = Self::prg_bank_offset(-1, cartridge);
        mapper.prg_offsets[1] = offset;
        mapper
    }

    fn prg_bank_offset(mut index: i32, cartridge: &Cartridge) -> i32 {
        if index >= 0x80 {
            index -= 0x100;
        }
        index %= (cartridge.prg.len() / 0x4000) as i32;
        let mut offset = index * 0x4000;
        if offset < 0 {
            offset += cartridge.prg.len() as i32;
        }
        offset
    }
}
